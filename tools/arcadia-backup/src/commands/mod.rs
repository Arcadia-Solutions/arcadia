pub mod backup;
pub mod check;
pub mod extract;
pub mod init;
pub mod prune;
pub mod restore;
pub mod snapshots;

use crate::config::Config;
use crate::rest_server::{self, RestServer};
use crate::shell::{quote, Secret};
use crate::ssh::{RemoteLock, Session};
use anyhow::Result;

/// Everything a remote run holds. The fields drop in declaration order: lock, then ssh, then rest-server.
pub struct Connection {
    #[expect(dead_code, reason = "held for its Drop")]
    pub lock: RemoteLock,
    pub session: Session,
    pub server: RestServer,
}

pub fn connect(config: &Config) -> Result<Connection> {
    eprintln!("--> connecting to {}", config.ssh.target);
    let server = RestServer::start(&config.restic.rest_server, &config.restic.repository)?;
    let session = Session::open(&config.ssh, server.port)?;
    let lock = RemoteLock::acquire(&session, &config.work_dir())?;
    Ok(Connection {
        lock,
        session,
        server,
    })
}

pub fn restic_secrets<'a>(password: &'a str, server: &'a RestServer) -> [Secret<'a>; 3] {
    [
        Secret {
            name: "RESTIC_PASSWORD",
            value: password,
        },
        Secret {
            name: "RESTIC_REST_USERNAME",
            value: rest_server::USERNAME,
        },
        Secret {
            name: "RESTIC_REST_PASSWORD",
            value: &server.password,
        },
    ]
}

pub fn git_commit(config: &Config, session: &Session) -> Result<String> {
    let output = session.run(
        &format!(
            "git -C {} rev-parse HEAD 2>/dev/null || echo unknown",
            quote(config.dir())
        ),
        &[],
        None,
    )?;
    Ok(output.trim().to_string())
}

/// Removes a directory of the Arcadia host once dropped, whatever happened
pub struct RemoteDirGuard<'a> {
    pub session: &'a Session,
    pub path: String,
}

impl Drop for RemoteDirGuard<'_> {
    fn drop(&mut self) {
        if let Err(error) = self
            .session
            .run(&format!("rm -rf {}", quote(&self.path)), &[], None)
        {
            eprintln!(
                "warning: cannot remove {} on the Arcadia host ({error:#}), it holds database dumps and configuration: remove it by hand",
                self.path
            );
        }
    }
}
