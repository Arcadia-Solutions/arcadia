/* KiwiIRC plugin: Arcadia customizations */
kiwi.plugin('arcadia', function (kiwi) {
    /* ── Autoconnect ─────────────────────────────────────────────────────
       Read connection config from window.name (set by the parent iframe)
       and apply it to the startup options so the client connects automatically.
    */
    var config;
    try {
        config = JSON.parse(window.name);
    } catch (e) {
        // no config in window.name
    }

    if (config) {
        window.name = '';
        var opts = kiwi.state.settings.startupOptions;
        Object.assign(opts, config, {
            direct_path: config.path || opts.direct_path,
        });
        !config.password && delete opts.password;
        !config.channel && delete opts.channel;
    }

    /* ── Custom CSS ───────────────────────────────────────────────────────
       Inject styles that override KiwiIRC defaults for Arcadia.
    */
    var style = document.createElement('style');
    style.textContent = '.kiwi-messagelist-message-notice { display: none !important; }';
    document.head.appendChild(style);
});
