First, thanks for considering contributing to Arcadia's devevelopment !

## Developer Setup

Arcadia's backend is a [REST](https://en.wikipedia.org/wiki/REST) API written in rust with the [actix](https://github.com/actix/actix-web) framework and the [sqlx](https://github.com/launchbadge/sqlx) database driver. It is made of 2 main parts : the site's API and a tracker. The site's API is meant to be used by the frontend, while the tracker is meant to be used by torrent clients (qbittorrent, deluge, etc.).

It (will) also contain some scripts that are meant to be run on a regular basis (for example as cron jobs) when the site is on production.

### Recommended tools
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [docker](https://docs.docker.com/desktop/setup/install)
- [insomnia](https://github.com/Kong/insomnia/)

### Environment
At runtime, arcadia's backend will source environment variables to influence it's behavior. The simplest way to set these during development is to write them into a file named `.env`.  A documented sample file is made available, so a quick way to get started is to use it by running `cp .env.example .env`.

### Database

Arcadia's backend uses a postgresql database. The recommended method for spawning an instance of postgres is using docker compose:

```bash
docker compose up db -d
```

> [!NOTE]
> If running `docker compose` doesn't work, you may have an older version of the docker cli installed and may need to use `docker-compose` instead.
> ```bash
> docker-compose up db -d
> ```

Arcadia will automatically run migrations on launch. Otherwise, initialization of the database can be done with:

```bash
cargo install sqlx-cli
cargo sqlx database setup
```

### Test data

You can optionally add "fake" data to the database by running the `fixtures.sql` file in the migrations/fixtures directory. This allows to quickly have data to work with.

Here is how to insert them if you are using docker :

```bash
docker exec -i arcadia_db psql -U arcadia -d arcadia < backend/migrations/fixtures/fixtures.sql
```

The default user defined in the test data is:

```
username: picolo
password: test
```

If you wish to create your own additional test data, you can then dump it to a file with this command (if you want to share it in the repo):

```bash
docker exec -i arcadia_db pg_dump -U arcadia -d arcadia --data-only > backend/migrations/fixtures/fixtures.sql
```

### Launch server

```bash
cargo run
```

This will start the development server in dev mode.

Alternatively you can also use docker compose to start the server:

```bash
docker compose up backend -d
```

It may take a while for the image to build as everything is done in docker. Yes, docker compose can handle that.
What docker compose can also handle is auto rebuilding images when source code changes as made possible by [compose watch](https://docs.docker.com/compose/how-tos/file-watch/).
To take advantage of that, just run this command:

```bash
docker compose up backend --watch
```

Or when running attached (as in not just turning it on and leaving it with the `--detach` / `-d` option), just press <kbd>W</kbd>.

Now when you make changes to the backend, compose will automatically rebuild the image and restart the container with the new source code, making (somewhat) quicker to iterate while also testing in docker.

## Code structure

API calls are forwarded to `handlers`, database requests are done by `repositories`, objects are defined by `models`. Directories with those names contain the relevant code.

A swagger for the API is available at `http://localhost:8080/swagger-ui/`

If you make changes to structs that are listed in the swagger, you must regenerate the typescript interfaces with this command (from the frontend directory, while the backend is running) :

```
npx openapi-typescript http://127.0.0.1:8080/swagger-json/openapi.json -o ./src/api-schema/schema.d.ts
```

## Testing

Adding additional tests to Arcadia is strongly encouraged, especially when adding new features! For unit tests, they can be added in the module being tested using standard rust idioms.

End-to-end tests can also be authored, they should be located in `tests/` and use the sqlx test fixture machinery to populate the database for testing.  See `tests/test_auth.rs` for examples.

## Contributing

Whether you want to add a new feature or fix an existing issue, it needs to be done on your own branch :

- fork this repo
- clone it locally on your computer
- create a new branch `feature-name` or `bug-name-fix` (with the proper name)
- open a pull request when your contribution is done

If you are unsure about what/how to do something, don't hesitate to [open a discussion](https://github.com/Arcadia-Solutions/arcadia/discussions) or [an issue](https://github.com/Arcadia-Solutions/arcadia/issues) about the topic.

You can also hop on the [discord server](https://discord.gg/amYWVk7pS3) to chat with other devs and the community.

### Finding contributions to make

Arcadia's backend has a [board](https://github.com/orgs/Arcadia-Solutions/projects/1) to track the existing issues and features that need to be worked on. Feel free to claim one that isn't claimed yet before starting to work on it.

To claim a github issue, simply leave a comment on it saying that you are working on it.

You can also search for `TODO` in the code and pick one of those tasks. If you decide to do this, please open an issue first and clam it before working on the task.
