import { config } from '@/config'

/** Connection settings read by the Arcadia KiwiIRC plugin from `window.name`. */
export const buildIrcConnectionConfig = (nick: string, password: string, channels: string[]) => {
  const websocketUrl = new URL(config.irc_websocket_url || '/webirc/websocket', window.location.origin)
  const tls = websocketUrl.protocol === 'https:' || websocketUrl.protocol === 'wss:'
  const port = websocketUrl.port || (tls ? '443' : '80')
  const path = websocketUrl.pathname === '/' ? '' : websocketUrl.pathname
  return { server: websocketUrl.hostname, port: parseInt(port, 10), tls, path, nick, password, channel: channels.join(',') }
}

/** Guests join without an account, so ergo must not have `require-sasl` enabled. */
export const guestIrcEnabled = () => (config.irc_webchat_guest_channels?.length ?? 0) > 0

/**
 * Opens the Kiwi IRC webchat in a new tab, connected as an anonymous `Guest-<number>`.
 * `window.open` names the new tab, which is how the plugin receives the connection config.
 */
export const openGuestIrcWebchat = () => {
  const nick = `Guest-${Math.floor(Math.random() * 1000000)}`
  const connection = buildIrcConnectionConfig(nick, '', config.irc_webchat_guest_channels ?? [])
  window.open(config.irc_webchat_url || '/kiwiirc/', JSON.stringify(connection))
}
