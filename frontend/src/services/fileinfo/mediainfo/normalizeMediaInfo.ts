import type { MediaInfoParsed } from './parseMediaInfo'

/** Generic key/value row used in General/Video/Audio tables */
export interface KV {
  label: string
  value: string
}

/** Final normalized structure for UI rendering */
export interface Summary {
  general: KV[]
  video: KV[]
  audio?: KV[] // optional detailed rows for the first audio
  audioLine?: string // headline of first audio (backwards compat)
  audioLines?: string[] // all audio tracks (#1, #2, ...)
}

/**
 * Normalize MediaInfo into UI-friendly summary.
 * Accepts either:
 *  - MediaInfo JSON (from --Output=JSON)
 *  - Parsed Text (from parseMediaInfo)
 */
export function normalizeMediaInfo(input: unknown): Summary {
  if (isMediaInfoJson(input)) {
    return fromJson(input)
  }
  // Validate or document: assumes input is MediaInfoParsed
  return fromParsedText(input as MediaInfoParsed)
}

/* ---------------- TYPES ---------------- */

/** JSON schema (simplified) */
interface MediaInfoJson {
  media: {
    track: Array<Record<string, string>>
  }
}

/** Type guard to check if input is JSON */
function isMediaInfoJson(obj: unknown): obj is MediaInfoJson {
  return typeof obj === 'object' && obj !== null && 'media' in obj && Array.isArray((obj as MediaInfoJson).media.track)
}

/* ---------------- JSON PATH ---------------- */

function fromJson(json: MediaInfoJson): Summary {
  const tracks = json.media.track
  const general = tracks.find((t) => t['@type'] === 'General') ?? {}
  const video = tracks.find((t) => t['@type'] === 'Video') ?? {}
  const audios = tracks.filter((t) => t['@type'] === 'Audio')

  const genRows: KV[] = [
    { label: 'Container', value: general.Format ?? '' },
    { label: 'Runtime', value: (humanDuration(general.Duration) || general['Duration/String']) ?? '' },
    { label: 'Size', value: general['FileSize/String4'] ?? general['FileSize/String3'] ?? general['FileSize/String'] ?? '' },
  ].filter((r) => r.value)

  const vRes = video.Width && video.Height ? `${video.Width}x${video.Height}` : ''
  const vidRows: KV[] = [
    { label: 'Codec', value: codecPretty(video) },
    { label: 'Resolution', value: vRes },
    { label: 'Aspect ratio', value: ratio(video.DisplayAspectRatio) },
    { label: 'Frame rate', value: fps(video.FrameRate) },
    { label: 'Bit rate', value: bitratePretty(video.BitRate) },
  ].filter((r) => r.value)

  const audioLines = audios.map((a, i) => buildAudioLineFromJson(a, i + 1))
  const firstAudio = audios[0] ?? {}
  const audioRows: KV[] = [
    { label: 'Language', value: langOf(firstAudio) },
    { label: 'Channels', value: channelPretty(firstAudio.Channels) },
    { label: 'Codec', value: codecPretty(firstAudio) },
    { label: 'Bit rate', value: bitratePretty(firstAudio.BitRate) },
  ].filter((r) => r.value)

  return { general: genRows, video: vidRows, audio: audioRows, audioLine: audioLines[0], audioLines }
}

/* ---------------- PARSED TEXT PATH ---------------- */

function fromParsedText(parsed: MediaInfoParsed): Summary {
  const g = parsed?.General?.[0]?.data ?? {}
  const v = parsed?.Video?.[0]?.data ?? {}
  const audioSections = parsed?.Audio ?? []

  const genRows: KV[] = [
    { label: 'Container', value: String(g['Format'] ?? '') },
    { label: 'Runtime', value: String(g['Duration'] ?? '') },
    { label: 'Size', value: String(g['File size'] ?? '') },
  ].filter((r) => r.value)

  const vidRows: KV[] = [
    { label: 'Codec', value: String(v['Format'] ?? v['Codec ID'] ?? '') },
    { label: 'Resolution', value: typeof v['Width'] === 'string' && typeof v['Height'] === 'string' ? `${digits(v['Width'])}x${digits(v['Height'])}` : '' },
    { label: 'Aspect ratio', value: String(v['Display aspect ratio'] ?? '') },
    { label: 'Frame rate', value: Array.isArray(v['Frame rate']) ? frameRate(v['Frame rate'][0]) : frameRate(v['Frame rate']) },
    { label: 'Bit rate', value: bitratePretty(Array.isArray(v['Bit rate']) ? v['Bit rate'][0] : v['Bit rate']) },
  ].filter((r) => r.value)

  const audioLines = audioSections.map((s: { data: Record<string, string | string[]> }, i: number) => buildAudioLineFromText(s.data, i + 1))
  const a0 = audioSections[0]?.data ?? {}
  const audioRows: KV[] = [
    { label: 'Language', value: String(a0['Language'] ?? '') },
    { label: 'Channels', value: channelPretty(Array.isArray(a0['Channel(s)']) ? a0['Channel(s)'].join(', ') : a0['Channel(s)']) },
    { label: 'Codec', value: String(a0['Format'] ?? a0['Codec ID'] ?? '') },
    { label: 'Bit rate', value: String(a0['Bit rate'] ?? '') },
  ].filter((r) => r.value)

  return { general: genRows, video: vidRows, audio: audioRows, audioLine: audioLines[0], audioLines }
}

/* ---------------- HELPERS ---------------- */

function humanDuration(msLike?: string): string {
  if (!msLike) return ''
  const ms = Number(msLike)
  if (!isFinite(ms) || ms <= 0) return ''
  const totalSec = Math.round(ms / 1000)
  const h = Math.floor(totalSec / 3600)
  const m = Math.floor((totalSec % 3600) / 60)
  return h ? `${h}h ${String(m).padStart(2, '0')}mn` : `${m}mn`
}
function ratio(v?: string): string {
  return v ? `${v}:1`.replace('::1', ':1') : ''
}
function fps(v?: string): string {
  return v ? `${v} fps` : ''
}
function bitratePretty(v?: string): string {
  return v ? v.replace(/bits\/second/i, 'b/s') : ''
}
function frameRate(v?: string): string {
  const num = v?.split(' ')[0]
  if (!num || isNaN(Number(num))) return ''
  return `${num} fps`
}
function digits(s: string): string {
  return s.replace(/\D+/g, '')
}
function langOf(a: Record<string, string>): string {
  return a.Language ?? 'Unknown'
}
function channelPretty(ch?: string): string {
  return ch ? ch.replace(/channels?/i, 'ch') : ''
}
function codecPretty(t: Record<string, string>): string {
  return t.Format ?? t['Codec ID'] ?? ''
}

function buildAudioLineFromJson(a: Record<string, string>, idx: number): string {
  const lang = a.Language ?? 'Unknown'
  const ch = channelPretty(a.Channels)
  const cdc = codecPretty(a)
  const br = bitratePretty(a.BitRate)
  const note = a.Title ?? ''
  const tail = [br && `@ ${br}`, note && `(${note})`].filter(Boolean).join(' ')
  return `#${idx}: ${[lang, ch, cdc].filter(Boolean).join(' ')}${tail ? ' ' + tail : ''}`
}
function buildAudioLineFromText(a: Record<string, string | string[]>, idx: number): string {
  const lang = Array.isArray(a['Language']) ? a['Language'].join(', ') : (a['Language'] ?? 'Unknown')
  const ch = channelPretty(Array.isArray(a['Channel(s)']) ? a['Channel(s)'].join(', ') : a['Channel(s)'])
  const cdc = Array.isArray(a['Format']) ? a['Format'].join(', ') : (a['Format'] ?? a['Codec ID'] ?? '')
  const br = Array.isArray(a['Bit rate']) ? a['Bit rate'].join(', ') : (a['Bit rate'] ?? '')
  const note = Array.isArray(a['Title']) ? a['Title'].join(', ') : (a['Title'] ?? '')
  const tail = [br && `@ ${br}`, note && `(${note})`].filter(Boolean).join(' ')
  return `#${idx}: ${[lang, ch, cdc].filter(Boolean).join(' ')}${tail ? ' ' + tail : ''}`
}
