/* KiwiIRC plugin: Arcadia customizations */
(function () {
    if (typeof kiwi === 'undefined') {
        return;
    }

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

    // Clear window.name so credentials don't persist
    window.name = '';

    if (config && config.nick && config.server) {
        kiwi.on('init', function () {
            var opts = kiwi.state.settings.startupOptions;
            opts.server = config.server;
            opts.nick = config.nick;
            opts.password = config.password || '';
            if (config.port) opts.port = config.port;
            if (config.tls !== undefined) opts.tls = config.tls;
            if (config.path) opts.path = config.path;
            if (config.channel) opts.channel = config.channel;
            opts.autoConnect = true;
            opts.direct = true;
        });
    }

    /* ── Custom CSS ───────────────────────────────────────────────────────
       Inject styles that override KiwiIRC defaults for Arcadia.
    */
    var style = document.createElement('style');
    style.textContent = [
        /* Hide server notices (e.g. "You are now logged in as ...") */
        '.kiwi-messagelist-message-notice { display: none !important; }',
    ].join('\n');
    document.head.appendChild(style);
})();
