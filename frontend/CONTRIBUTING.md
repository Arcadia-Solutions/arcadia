First, thanks for considering contributing to Arcadia !

## Developer setup

### Recommended tools :

- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [prettier](https://prettier.io)
- [docker](https://docs.docker.com/engine/install/) (or equivalent, e.g. podman, orbstack)


### Run the project :

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

This will launch a local server for the frontend

Launch the backend:
```
# from the project root
docker-compose up -d
```

Please note that this will perform a build the first time the command is run, which may take a while. Changes to the Dockerfile or backend files will not trigger a rebuild. If you want to pull in new changes, run:

```
docker-compose up --build
```


## Contributing

Wheter you want to add a new feature or fix an existing issue, it needs to be done on your own branch :

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
