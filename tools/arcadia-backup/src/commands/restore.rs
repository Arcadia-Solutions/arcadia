use crate::commands::{self, Connection};
use crate::components::{self, extra, files, Restored};
use crate::config::{Config, MariadbWithFiles, Mode, Runner, StandardConfig};
use crate::ctx::{compose, in_dir, parse_running, root_container, Ctx};
use crate::meta::Meta;
use crate::restic::{LocalRestic, RemoteRestic};
use crate::selector::Selector;
use crate::shell::{self, quote};
use crate::ssh::Session;
use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::io::Write;
use std::time::Duration;

pub struct Options {
    pub snapshot: Selector,
    pub only: Vec<String>,
    pub skip_config: bool,
    pub yes: bool,
}

/// Services using the data being restored, stopped during the restore (docker mode)
pub const APP_SERVICES: [&str; 6] = [
    "backend",
    "tracker",
    "frontend",
    "chevereto_php",
    "ergo",
    "kiwiirc",
];

pub fn selected_components(
    meta: &Meta,
    only: &[String],
    skip_config: bool,
) -> Result<Vec<&'static str>> {
    let known = components::names();
    let available: Vec<&str> = meta.components.keys().map(String::as_str).collect();
    if let Some(unknown) = available.iter().find(|name| !known.contains(name)) {
        bail!(
            "the snapshot holds the component '{unknown}', which this arcadia-backup version does not know"
        );
    }
    for name in only {
        if !available.contains(&name.as_str()) {
            bail!(
                "the snapshot has no component '{name}', it has: {}",
                available.join(", ")
            );
        }
    }
    let selected: Vec<&'static str> = known
        .into_iter()
        .filter(|name| available.contains(name))
        .filter(|name| only.is_empty() || only.iter().any(|o| o == name))
        .filter(|name| !(skip_config && *name == "config"))
        .collect();
    if selected.is_empty() {
        bail!("nothing to restore: --only and --skip-config leave no component selected");
    }
    Ok(selected)
}

/// What a failed restore did and did not restore, as two lines
pub fn restore_report(selected: &[&str], done: &[&str]) -> [String; 2] {
    let list = |names: Vec<&str>| {
        if names.is_empty() {
            "nothing".to_string()
        } else {
            names.join(", ")
        }
    };
    let not_done = selected.iter().copied().filter(|n| !done.contains(n));
    [
        format!("restored: {}", list(done.to_vec())),
        format!("not restored: {}", list(not_done.collect())),
    ]
}

/// A host on which `docker compose ps` fails is only a new host, where nothing runs, when it
/// has no configuration and no container of the project. Anything else is an unknown state:
/// restoring over possibly running services is refused.
pub fn fresh_host_check(
    config_yml_exists: bool,
    project_containers: &str,
    dir: &str,
) -> Result<()> {
    if config_yml_exists {
        bail!("{dir}/config.yml exists, so this is not a new host");
    }
    if !project_containers.trim().is_empty() {
        bail!("containers of the compose project in {dir} exist");
    }
    Ok(())
}

/// Requirements of the selected components that the configuration alone tells, standard mode.
/// Docker mode needs nothing more than the snapshot.
pub fn static_problems(
    mode: Mode,
    selected: &[&str],
    standard: Option<&StandardConfig>,
) -> Vec<String> {
    let mut problems = Vec::new();
    if mode != Mode::Standard {
        return problems;
    }
    for name in ["chevereto", "ergo"] {
        if selected.contains(&name) && standard.and_then(|s| mariadb_section(s, name)).is_none() {
            problems.push(format!(
                "the snapshot holds {name}: add the standard.{name} section (database and dir) to restore it, or skip it with --only"
            ));
        }
    }
    if selected.contains(&"redis") && standard.and_then(|s| s.redis.as_ref()).is_none() {
        problems.push(
            "the snapshot holds redis: set standard.redis.rdb_path to restore it, or skip it with --only"
                .to_string(),
        );
    }
    problems
}

fn mariadb_section<'a>(standard: &'a StandardConfig, name: &str) -> Option<&'a MariadbWithFiles> {
    match name {
        "chevereto" => standard.chevereto.as_ref(),
        _ => standard.ergo.as_ref(),
    }
}

/// Password files of the selected components, to be readable on the Arcadia host
pub fn password_files(selected: &[&str], standard: Option<&StandardConfig>) -> Vec<String> {
    let Some(standard) = standard else {
        return Vec::new();
    };
    ["chevereto", "ergo"]
        .into_iter()
        .filter(|name| selected.contains(name))
        .filter_map(|name| mariadb_section(standard, name))
        .map(|section| section.database.password_file.clone())
        .collect()
}

fn has_parent_component(key: &str) -> bool {
    key.split('/').any(|part| part == "..")
}

/// The keys of meta.json become paths on the Arcadia host: only the ones this version would
/// have written for the current configuration are restored
pub fn meta_key_problems(meta: &Meta, selected: &[&str], config: &Config) -> Vec<String> {
    let extra_keys: Vec<String> = extra::includes(&config.extra)
        .into_iter()
        .map(|include| include.key)
        .collect();
    let mut problems = Vec::new();
    for name in selected {
        let fixed: Option<&[&str]> = match *name {
            "config" => Some(files::CONFIG.paths),
            "custom_content" => Some(files::CUSTOM_CONTENT.paths),
            _ => None,
        };
        for item in meta
            .components
            .get(*name)
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            if has_parent_component(&item.key) {
                problems.push(format!("{name}: the key '{}' contains '..'", item.key));
            } else if fixed.is_some_and(|paths| !paths.contains(&item.key.as_str())) {
                problems.push(format!(
                    "{name}: the snapshot holds '{}', which is not one of the files of this component",
                    item.key
                ));
            } else if *name == "extra" && !extra_keys.contains(&item.key) {
                problems.push(format!(
                    "extra: the snapshot holds '{}', which is neither in extra.paths nor in extra.volumes of the configuration",
                    item.key
                ));
            }
        }
    }
    problems
}

pub fn services_to_stop(running: &BTreeSet<String>) -> Vec<String> {
    APP_SERVICES
        .iter()
        .filter(|service| running.contains(**service))
        .map(|service| service.to_string())
        .collect()
}

/// Database services to start before restoring; redis stops and starts its own service
pub fn data_services(selected: &[&str]) -> Vec<&'static str> {
    selected
        .iter()
        .filter_map(|name| match *name {
            "postgres" => Some("db"),
            "chevereto" => Some("chevereto_database"),
            "ergo" => Some("ergo_database"),
            _ => None,
        })
        .collect()
}

/// Empties (creating it if needed) the restore root; the docker runner restores as root, so it
/// is emptied from a root container. Only the last line may be an `&&` list: a failure of an
/// earlier step must abort the script.
pub fn clear_script(config: &Config, root: &str) -> Result<String> {
    let root_q = quote(root);
    let recreate = format!("mkdir -p {root_q} && chmod 700 {root_q}");
    match config.restic.runner {
        Runner::Binary => Ok(format!("rm -rf {root_q} && {recreate}")),
        Runner::Docker => {
            let clear = root_container(
                config,
                &[format!("{root}:/restore")],
                "find /restore -mindepth 1 -delete",
            )?;
            Ok(format!("if [ -d {root_q} ]; then {clear}; fi\n{recreate}"))
        }
    }
}

struct RestoreRootGuard<'a> {
    config: &'a Config,
    session: &'a Session,
    root: String,
}

impl Drop for RestoreRootGuard<'_> {
    fn drop(&mut self) {
        if let Ok(clear) = clear_script(self.config, &self.root) {
            let script = format!("{clear}\nrmdir {}", quote(&self.root));
            if let Err(error) = self.session.run(&script, &[], None) {
                eprintln!(
                    "warning: cannot remove {} on the Arcadia host ({error:#}), it holds database dumps and configuration: remove it by hand",
                    self.root
                );
            }
        }
    }
}

fn confirm() -> Result<bool> {
    print!("Type yes to go on: ");
    std::io::stdout().flush()?;
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    Ok(answer.trim() == "yes")
}

/// The running application services to stop. When compose cannot list them, the host is only
/// taken for a new one (nothing runs there) on positive evidence, see `fresh_host_check`.
fn services_running(config: &Config, session: &Session) -> Result<Vec<String>> {
    let listing = in_dir(
        config.dir(),
        &compose(&["ps", "--services", "--status", "running"]),
    );
    let running = match session.run(&listing, &[], None) {
        Ok(output) => parse_running(&output),
        Err(error) => {
            let dir = config.dir();
            let config_yml = session.run(
                &format!(
                    "if [ -e {} ]; then echo yes; else echo no; fi",
                    quote(&format!("{dir}/config.yml"))
                ),
                &[],
                None,
            )?;
            let filter = format!("label=com.docker.compose.project.working_dir={dir}");
            let containers = session.run(
                &shell::join(&["docker", "ps", "-a", "-q", "--filter", &filter]),
                &[],
                None,
            )?;
            fresh_host_check(config_yml.trim() == "yes", &containers, dir).with_context(|| {
                format!(
                    "cannot list the running services ({error:#}), and the host does not look new; nothing was modified"
                )
            })?;
            eprintln!("warning: cannot list the running services ({error:#}), this looks like a new host: assuming none runs");
            BTreeSet::new()
        }
    };
    Ok(services_to_stop(&running))
}

fn stop_app_services(config: &Config, session: &Session, services: &[String]) -> Result<()> {
    if !services.is_empty() {
        eprintln!("--> stopping {}", services.join(", "));
        let mut args = vec!["stop"];
        args.extend(services.iter().map(String::as_str));
        session.run(&in_dir(config.dir(), &compose(&args)), &[], None)?;
    }
    Ok(())
}

/// Everything that can be checked before the first change. Returns the services to stop.
fn preflight(
    config: &Config,
    session: &Session,
    meta: &Meta,
    selected: &[&'static str],
) -> Result<Vec<String>> {
    let standard = config.standard.as_ref();
    let mut problems = static_problems(config.arcadia.mode, selected, standard);
    problems.extend(meta_key_problems(meta, selected, config));
    if config.arcadia.mode == Mode::Standard && problems.is_empty() {
        let files = password_files(selected, standard);
        if !files.is_empty() {
            let refs: Vec<&str> = files.iter().map(String::as_str).collect();
            let unreadable = session.run(
                &format!(
                    "for f in {}; do if [ ! -r \"$f\" ]; then echo \"$f\"; fi; done",
                    shell::join(&refs)
                ),
                &[],
                None,
            )?;
            problems.extend(unreadable.lines().map(|file| {
                format!("the password file {file} is not readable on the Arcadia host")
            }));
        }
    }
    if !problems.is_empty() {
        bail!(
            "the restore cannot work, nothing was modified:\n  - {}",
            problems.join("\n  - ")
        );
    }
    match config.arcadia.mode {
        Mode::Docker => services_running(config, session),
        Mode::Standard => Ok(Vec::new()),
    }
}

fn restore_components(
    ctx: &Ctx,
    meta: &Meta,
    selected: &[&'static str],
    root: &str,
    done: &mut Vec<&'static str>,
) -> Result<()> {
    for component in components::all() {
        let name = component.name();
        if name == "config" || !selected.contains(&name) {
            continue;
        }
        eprintln!("--> {name}");
        let restored = Restored {
            root,
            items: &meta.components[name],
        };
        component
            .restore(ctx, &restored)
            .with_context(|| format!("cannot restore {name}"))?;
        done.push(name);
    }
    Ok(())
}

fn restart(config: &Config, session: &Session, services: &[String]) -> Result<()> {
    if services.is_empty() {
        println!("No application service was running. Start the stack, e.g.: docker compose --profile full up -d --build");
        return Ok(());
    }
    // --build: the frontend image inlines config.yml and copies frontend/public at build time
    eprintln!("--> starting {} again", services.join(", "));
    let mut args = vec!["up", "-d", "--build"];
    args.extend(services.iter().map(String::as_str));
    session.run(&in_dir(config.dir(), &compose(&args)), &[], None)?;
    if services.iter().any(|service| service == "backend") {
        eprintln!("--> waiting for the backend to be healthy");
        let health = in_dir(
            config.dir(),
            &compose(&[
                "exec",
                "-T",
                "backend",
                "curl",
                "-fsS",
                "-o",
                "/dev/null",
                "http://localhost:8080/health",
            ]),
        );
        for _ in 0..60 {
            if session.run(&health, &[], None).is_ok() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_secs(2));
        }
        bail!("the backend did not become healthy within 2 minutes, check `docker compose logs backend`");
    }
    Ok(())
}

pub fn run(config: &Config, options: Options) -> Result<()> {
    let password = config.read_password()?;
    let host = config.host_name();
    let local = LocalRestic::new(config);
    let snapshots = local.snapshots(&host)?;
    let snapshot = options.snapshot.resolve(&snapshots)?.clone();
    let meta = Meta::from_json(&local.meta_json(&snapshot.id, &config.work_dir())?)?;
    let selected = selected_components(&meta, &options.only, options.skip_config)?;

    let connection: Connection = commands::connect(config)?;
    let session = &connection.session;
    let warnings =
        meta.check_target(config.arcadia.mode, &commands::git_commit(config, session)?)?;

    println!(
        "Snapshot {} of {host}, made on {}",
        snapshot.short_id,
        snapshot.time.format("%Y-%m-%d %H:%M:%S %z")
    );
    println!(
        "Restoring, and overwriting on {}: {}",
        config.ssh.target,
        selected.join(", ")
    );
    for warning in &warnings {
        println!("warning: {warning}");
    }
    if config.arcadia.mode == Mode::Standard {
        println!("Stop the Arcadia services (backend, tracker, frontend, image host, irc) and redis before going on.");
    }
    if !options.yes && !confirm()? {
        bail!("restore cancelled");
    }

    let stop = preflight(config, session, &meta, &selected)?;

    let root = format!("{}/restore", config.work_dir());
    session.run(&clear_script(config, &root)?, &[], None)?;
    let _cleanup = RestoreRootGuard {
        config,
        session,
        root: root.clone(),
    };
    eprintln!("--> restic restore {}", snapshot.short_id);
    let remote = RemoteRestic {
        config,
        port: session.remote_port,
    };
    session.run(
        &remote.restore_script(&snapshot.id, &root)?,
        &commands::restic_secrets(&password, &connection.server),
        None,
    )?;

    stop_app_services(config, session, &stop)?;

    let mut done: Vec<&'static str> = Vec::new();
    let result = (|| -> Result<()> {
        if selected.contains(&"config") {
            eprintln!("--> config");
            files::CONFIG.restore_with(
                config,
                session,
                &Restored {
                    root: &root,
                    items: &meta.components["config"],
                },
            )?;
            done.push("config");
        }
        let ctx = Ctx::load(config, session)?;
        if config.arcadia.mode == Mode::Docker {
            let services = data_services(&selected);
            if !services.is_empty() {
                eprintln!("--> starting {}", services.join(", "));
                let mut args = vec!["up", "-d", "--wait"];
                args.extend(services);
                ctx.run(&in_dir(config.dir(), &compose(&args)), &[])?;
            }
        }
        restore_components(&ctx, &meta, &selected, &root, &mut done)
    })();

    if let Err(error) = result {
        for line in restore_report(&selected, &done) {
            eprintln!("{line}");
        }
        eprintln!("the application services are left stopped");
        return Err(error);
    }

    match config.arcadia.mode {
        Mode::Docker => restart(config, session, &stop).with_context(|| {
            format!(
                "the data restore of snapshot {} completed, only restarting the services failed",
                snapshot.short_id
            )
        })?,
        Mode::Standard => println!("Start redis and the Arcadia services again."),
    }
    println!("snapshot {} restored", snapshot.short_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{test_config, Mode, Runner};
    use crate::meta::{Item, Meta};

    fn meta(components: &[&str]) -> Meta {
        let mut meta = Meta::new(Mode::Docker, Runner::Docker, "c".into());
        for name in components {
            meta.components.insert(
                name.to_string(),
                vec![Item {
                    key: "k".into(),
                    snapshot_path: "/p".into(),
                }],
            );
        }
        meta
    }

    #[test]
    fn restores_everything_in_the_snapshot_in_registry_order() {
        let meta = meta(&["redis", "postgres", "config", "custom_content"]);
        assert_eq!(
            selected_components(&meta, &[], false).unwrap(),
            vec!["config", "custom_content", "postgres", "redis"]
        );
    }

    #[test]
    fn skip_config_and_only_narrow_the_selection() {
        let meta = meta(&["config", "postgres", "redis"]);
        assert_eq!(
            selected_components(&meta, &[], true).unwrap(),
            vec!["postgres", "redis"]
        );
        assert_eq!(
            selected_components(&meta, &["redis".to_string()], false).unwrap(),
            vec!["redis"]
        );
        let error = selected_components(&meta, &["chevereto".to_string()], false)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("no component 'chevereto'") && error.contains("config, postgres, redis"),
            "{error}"
        );
    }

    #[test]
    fn unknown_components_in_the_snapshot_are_rejected() {
        let error = selected_components(&meta(&["postgres", "from_the_future"]), &[], false)
            .unwrap_err()
            .to_string();
        assert!(error.contains("from_the_future"), "{error}");
    }

    #[test]
    fn services_to_stop_keeps_only_running_app_services() {
        let running = ["db", "redis", "backend", "ergo", "my_plugin"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(services_to_stop(&running), vec!["backend", "ergo"]);
        assert!(services_to_stop(&Default::default()).is_empty());
    }

    #[test]
    fn data_services_follow_the_selected_components() {
        assert_eq!(
            data_services(&["config", "postgres", "redis", "ergo"]),
            vec!["db", "ergo_database"]
        );
    }

    #[test]
    fn restore_root_is_emptied_as_root_with_the_docker_runner() {
        let script = clear_script(&test_config("docker", "docker"), "/w/restore").unwrap();
        assert!(
            script.contains("find /restore -mindepth 1 -delete"),
            "{script}"
        );
        assert!(script.contains("set -eu\n"), "{script}");
        assert!(
            script.ends_with("\nmkdir -p /w/restore && chmod 700 /w/restore"),
            "{script}"
        );
        assert_eq!(
            clear_script(&test_config("standard", "binary"), "/w/restore").unwrap(),
            "rm -rf /w/restore && mkdir -p /w/restore && chmod 700 /w/restore"
        );
    }

    const STANDARD: &str = "\
ssh:
  target: arcadia.example.com
arcadia:
  dir: /srv/arcadia
  mode: standard
restic:
  repository: /srv/backups/arcadia
  password_file: /etc/arcadia-backup/password
  runner: binary
retention:
  keep_last: 3
extra:
  paths: [/srv/plugin/data]
standard:
  chevereto:
    database: { user: c, password_file: /etc/c.pass, name: chevereto }
    dir: /var/www/chevereto/images
";

    fn standard_config() -> Config {
        Config::parse(STANDARD).unwrap()
    }

    fn meta_with(component: &str, keys: &[&str]) -> Meta {
        let mut meta = Meta::new(Mode::Standard, Runner::Binary, "c".into());
        meta.components.insert(
            component.to_string(),
            keys.iter()
                .map(|key| Item {
                    key: key.to_string(),
                    snapshot_path: format!("/p/{key}"),
                })
                .collect(),
        );
        meta
    }

    #[test]
    fn an_empty_selection_is_rejected() {
        let error = selected_components(&meta(&["config"]), &["config".to_string()], true)
            .unwrap_err()
            .to_string();
        assert!(error.contains("nothing to restore"), "{error}");
    }

    #[test]
    fn the_report_lists_what_was_and_was_not_restored() {
        let selected = ["config", "postgres", "redis"];
        assert_eq!(
            restore_report(&selected, &["config", "postgres"]),
            ["restored: config, postgres", "not restored: redis"]
        );
        assert_eq!(
            restore_report(&selected, &[]),
            ["restored: nothing", "not restored: config, postgres, redis"]
        );
        assert_eq!(
            restore_report(&selected, &selected),
            ["restored: config, postgres, redis", "not restored: nothing"]
        );
    }

    #[test]
    fn a_failing_compose_only_means_a_new_host_with_positive_evidence() {
        assert!(fresh_host_check(false, "\n", "/opt/arcadia").is_ok());
        let error = fresh_host_check(true, "", "/opt/arcadia")
            .unwrap_err()
            .to_string();
        assert!(error.contains("/opt/arcadia/config.yml exists"), "{error}");
        let error = fresh_host_check(false, "4f2a9c\n", "/opt/arcadia")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("containers of the compose project"),
            "{error}"
        );
    }

    #[test]
    fn standard_restores_need_their_configuration_sections() {
        let all = ["config", "postgres", "redis", "chevereto", "ergo"];
        let problems = static_problems(Mode::Standard, &all, standard_config().standard.as_ref());
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(problems.iter().any(|p| p.contains("standard.ergo")));
        assert!(problems
            .iter()
            .any(|p| p.contains("standard.redis.rdb_path")));
        assert!(static_problems(
            Mode::Standard,
            &["postgres", "chevereto"],
            standard_config().standard.as_ref()
        )
        .is_empty());
        assert_eq!(static_problems(Mode::Standard, &["ergo"], None).len(), 1);
        assert!(static_problems(Mode::Docker, &all, None).is_empty());
    }

    #[test]
    fn password_files_of_the_selected_components_are_checked() {
        let config = standard_config();
        assert_eq!(
            password_files(&["postgres", "chevereto", "ergo"], config.standard.as_ref()),
            vec!["/etc/c.pass"]
        );
        assert!(password_files(&["postgres"], config.standard.as_ref()).is_empty());
        assert!(password_files(&["chevereto"], None).is_empty());
    }

    #[test]
    fn meta_keys_must_be_known_and_stay_in_place() {
        let config = standard_config();
        let check = |component: &str, keys: &[&str]| {
            meta_key_problems(&meta_with(component, keys), &[component], &config)
        };
        assert!(check("config", &["config.yml", ".env"]).is_empty());
        assert!(check("custom_content", &["frontend/public/home"]).is_empty());
        assert!(check("extra", &["srv/plugin/data"]).is_empty());
        assert!(check("postgres", &["postgres.sql"]).is_empty());

        let problems = check("config", &["../../etc/cron.d/x", "config.yml", "other.yml"]);
        assert_eq!(problems.len(), 2, "{problems:?}");
        assert!(problems[0].contains("contains '..'"), "{problems:?}");
        assert!(problems[1].contains("'other.yml'"), "{problems:?}");

        assert_eq!(check("custom_content", &["etc/passwd"]).len(), 1);
        let problems = check(
            "extra",
            &["etc/cron.d", "volume/v", "srv/plugin/data/../../x"],
        );
        assert_eq!(problems.len(), 3, "{problems:?}");
        assert!(check("postgres", &["a/../b"])[0].contains("'..'"));
    }
}
