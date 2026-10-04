# IRC Server

kiwiirc (web client) and ergo (irc server) are not required to run the rest of Arcadia.
in order to use them you should create untracked copies of their configuration files

   ```bash
   cp kiwiirc/config.json.example kiwiirc/config.json # optional (uses example as fallback automatically)
   cp ergo/ergo.motd.example ergo/ergo.motd
   cp ergo/ergo-conf.yaml.example ergo/ergo-conf.yaml
   ```

The API tokens declared in `ergo/ergo-conf.yaml` must match the ones of the `ergo` section of
`config.yml`.
