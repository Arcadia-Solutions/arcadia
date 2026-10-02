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
        // Clear window.name so credentials don't persist across navigations.
        window.name = '';
        // Wipe any networks persisted in localStorage from a previous session.
        // The welcome screen skips autoConnect if networks.length > 0, so stale
        // state would cause it to reuse the old nick/connection instead of ours.
        kiwi.state.resetState();
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
    // Hide server notices (e.g. "You are now logged in as ...")
    style.textContent = '.kiwi-messagelist-message-notice { display: none !important; }';
    document.head.appendChild(style);
});
