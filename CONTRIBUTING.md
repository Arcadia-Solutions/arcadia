First, thanks for considering contributing to Arcadia !

## Developer setup

Arcadia's backend is a [REST](https://en.wikipedia.org/wiki/REST) API written in rust with the [actix](https://github.com/actix/actix-web) framework and the [sqlx](https://github.com/launchbadge/sqlx) database driver. It also uses [PostgreSQL](https://www.postgresql.org/) as it's database.
It is made of 2 main parts: the site's API and a tracker. The site's API is meant to be used by the frontend, while the tracker is meant to be used by torrent clients (qbittorrent, deluge, etc.).

It (will) also contain some scripts that are meant to be run on a regular basis (for example as cron jobs) when the site is on production.

Arcadia's frontend is a [SPA](https://developer.mozilla.org/en-US/docs/Glossary/SPA) written in [TypeScript](https://www.typescriptlang.org/) and uses the [Vue.js](https://vuejs.org/) framework with [Vite](https://vite.dev/) to build it.

> [!TIP]
> If you don't want to install another toolchain on your system, You can also use [devcontainers](https://containers.dev/) instead.
> If you don't know, think of isolated minimal virtual machines that come with the tools required to build Arcadia (or anything else really).
>
> If you have Docker (recommended!) installed and use [Visual Studio Code](https://code.visualstudio.com/), all you need is to have the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed and "reopening your folder in a container".
> You can find that option in your Command Palette, or by clicking on the new status bar item in the left bottom corner.
>
> You can also use GitHub Codespaces to build Arcadia in the cloud without having to download anything but streams of text, although it's not as free as a local dev container.
>
> [![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/Arcadia-Solutions/arcadia?quickstart=1)
>
> It isn't required to use them but can be useful in some cases (especially if you're using an immutable OS).

### Required Tools

You need these to make meaningful contributions to Arcadia, outside the cases of documentation for example.

- [Node.js & npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Recommended tools

- [Prettier](https://prettier.io) for proper formatting of the frontend's code.
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) for managing database related stuff, including migrations.
- [Docker](https://docs.docker.com/desktop/setup/install) for setting up dependencies. Optional but HIGHLY recommended!
- [Insomnia](https://github.com/Kong/insomnia/) for testing the backend's API. You could also use any other client if you want.


### Environment

#### Backend

At runtime, Arcadia's backend will source environment variables to influence it's behavior. The simplest way to set these during development is to write them into a file named `.env`.  A documented sample file is made available, so a quick way to get started is to use it by running `cp .env.example .env`.

#### Frontend

At build time, the frontend will be hardcoded with the site's API location sourced from an enviroment variable. A documented sample file is made available, so a quick way to get started is to use it by running `cp .env.example .env`.

### Run the project:

> [!NOTE]
> If running `docker compose` doesn't work, you may have an older version of the docker cli installed and may need to use `docker-compose` instead.
> ```bash
> docker-compose up db -d
> ```
>
> Also don't forgot to use `sudo` if you aren't in the `docker` group!

#### Backend

##### Dependencies

The recommended method for spawning an instance of PostgreSQL is using Docker Compose:

```bash
docker compose up db -d
```

Arcadia will automatically run migrations on launch. Otherwise, initialization of the database can be done with:

```bash
cargo sqlx database setup
# If cargo doesn't know the command, install it with this command
cargo install sqlx-cli
```

You can optionally add "fake" data to the database by running the `fixtures.sql` file in the migrations/fixtures directory. This allows to quickly have data to work with.

Here is how to insert them if you are using docker :

```bash
docker exec -i arcadia_db psql -U arcadia -d arcadia < backend/migrations/fixtures/fixtures.sql
```

The default user defined in the test data is `picolo` with password `test`.

If you wish to create your own additional test data, you can then dump it to a file with this command (if you want to share it in the repo):

```bash
docker exec -i arcadia_db pg_dump -U arcadia -d arcadia --data-only > backend/migrations/fixtures/fixtures.sql
```

You can then launch the server:

```bash
cargo run
```

This will start the server in dev mode.

Alternatively you can also use Docker Compose to start the server:

```bash
docker compose up backend -d
```

To stop the server using Docker Compose:

```bash
docker compose stop backend
```

If you need to see logs (and be able to conveniently stop it with <kbd>Control</kbd> + <kbd>C</kbd>), run it in attached mode:

```bash
docker compose up backend
```

It may take a while for the image to build as everything is done in Docker. Yes, Docker Compose can handle that!
What Docker Compose can also handle is auto rebuilding images when source code changes as made possible by [Compose Watch](https://docs.docker.com/compose/how-tos/file-watch/).
To take advantage of that, just run this command instead:

```bash
docker compose up backend --watch
```

Or when running attached (as in not just turning it on and leaving it with the `--detach` / `-d` option), just press <kbd>W</kbd>.

Now when you make changes to the backend, compose will automatically rebuild the image and restart the container with the new source code, making *somewhat* quicker to iterate while also testing in Docker.

#### Frontend

Make sure the [backend is running](#backend-1)!

Install the dependencies:

```bash
npm install
```

Create a `.env` file from the template and update the values if needed:

```bash
cp .env.example .env
```

Launch the server:

```bash
npm run dev
```

This will launch a local development server for the frontend.

## Testing

### Backend

Adding additional tests to Arcadia is strongly encouraged, especially when adding new features! For unit tests, they can be added in the module being tested using standard rust idioms.

End-to-end tests can also be authored, they should be located in `tests/` and use the sqlx test fixture machinery to populate the database for testing.  See `tests/test_auth.rs` for examples.

### Frontend

We don't have any tests for the frontend. I'm pretty sure we'll add them once it's more "production ready". (@shahoob)

## Contributing

Whether you want to add a new feature or fix an existing issue, it needs to be done on your own branch :

- fork this repo
- clone it locally on your computer
- create a new branch `feature-name` or `bug-name-fix` (with the proper name)
- open a pull request when your contribution is done

If you are unsure about what/how to do something, don't hesitate to [open a discussion](https://github.com/Arcadia-Solutions/arcadia/discussions) or [an issue](https://github.com/Arcadia-Solutions/arcadia/issues) about the topic.

You can also hop on the [discord server](https://discord.gg/amYWVk7pS3) to chat with other devs and the community.

### Finding contributions to make

Arcadia's frontend has a [board](https://github.com/orgs/Arcadia-Solutions/projects/2/views/1) to track the existing issues and features that need to be worked on. Feel free to claim one that isn't claimed yet before starting to work on it.

To claim a github issue, simply leave a comment on it saying that you are working on it.

You can also search for `TODO` in the code and pick one of those tasks. If you decide to do this, please open an issue first and clam it before working on the task.
