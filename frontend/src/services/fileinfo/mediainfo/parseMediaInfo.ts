/**
 * Represents a single section in MediaInfo Text output
 * Example: "Video", "Audio", "Text #1", etc.
 */
export interface MediaInfoSection {
  header: string // Full header line (e.g. "Text #1")
  base: string // Section base name (e.g. "Text")
  index?: number // Track number if present
  data: Record<string, string | string[]> // Key/value pairs for this section
}

/**
 * The full parsed MediaInfo structure:
 * - Keys: section types (General, Video, Audio, Text, ...)
 * - Values: array of sections (because there can be multiple Audio/Text tracks)
 */
export type MediaInfoParsed = Record<string, MediaInfoSection[]>

/**
 * Parse MediaInfo --Output=Text into a structured object.
 * Splits into sections, extracts key/value pairs, handles duplicate keys.
 */
export function parseMediaInfo(text: string): MediaInfoParsed {
  if (!text) return {}

  // Some inputs come escaped with "\n". If so, unescape once.
  const looksEscaped = /\\n/.test(text)
  const normalized = (looksEscaped ? text.replace(/\\n/g, '\n') : text).replace(/\r\n?/g, '\n').trim()

  // Split into blocks separated by blank lines
  const blocks = normalized.split(/\n\s*\n+/)

  const result: MediaInfoParsed = {}

  for (const block of blocks) {
    const lines = block
      .split('\n')
      .map((l) => l.trim())
      .filter(Boolean)
    if (!lines.length) continue

    const header = lines[0]
    const match = header.match(/^(.+?)\s*(?:#(\d+))?$/)
    const base = match?.[1]?.trim() ?? header
    const index = match?.[2] ? Number(match[2]) : undefined

    const data: Record<string, string | string[]> = {}

    for (let i = 1; i < lines.length; i++) {
      const line = lines[i]
      if (!line.includes(':')) continue

      // Only split on the first colon
      const [rawKey, ...rest] = line.split(':')
      const key = rawKey.trim()
      const value = rest.join(':').trim()

      if (!key) continue

      // If key already exists, merge into array
      if (key in data) {
        const existing = data[key]
        data[key] = Array.isArray(existing) ? [...existing, value] : [existing, value]
      } else {
        data[key] = value
      }
    }

    const section: MediaInfoSection = { header, base, index, data }
    ;(result[base] ??= []).push(section)
  }

  return result
}
