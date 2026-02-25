import { uniq, compact } from 'lodash-es'
import { VIDEO_OPTION } from '../utils'
import type { ParseResult } from './mediainfoParser'
import {
  Language as LanguageEnum,
  VideoResolution as VideoResolutionEnum,
  VideoCodec as VideoCodecEnum,
  AudioCodec as AudioCodecEnum,
  AudioChannels as AudioChannelsEnum,
} from '@/services/api-schema/api'

type VideoResolution = string | [string, string]

const languageValues = new Set<string>(Object.values(LanguageEnum))
const videoResolutionValues = new Set<string>(Object.values(VideoResolutionEnum))
const videoCodecValues = new Set<string>(Object.values(VideoCodecEnum))
const audioCodecValues = new Set<string>(Object.values(AudioCodecEnum))
const audioChannelsValues = new Set<string>(Object.values(AudioChannelsEnum))

/** Map MediaInfo language strings that don't directly match our enum */
const languageAliases: Record<string, string> = {
  // ISO 639-1 codes
  en: 'English',
  fr: 'French',
  de: 'German',
  es: 'Spanish',
  it: 'Italian',
  pt: 'Portuguese',
  ru: 'Russian',
  ja: 'Japanese',
  ko: 'Korean',
  zh: 'Chinese',
  ar: 'Arabic',
  hi: 'Hindi',
  nl: 'Dutch',
  pl: 'Polish',
  sv: 'Swedish',
  da: 'Danish',
  fi: 'Finnish',
  no: 'Norwegian',
  cs: 'Czech',
  hu: 'Hungarian',
  ro: 'Romanian',
  el: 'Greek',
  tr: 'Turkish',
  th: 'Thai',
  vi: 'Vietnamese',
  uk: 'Ukrainian',
  he: 'Hebrew',
  hr: 'Croatian',
  sr: 'Serbian',
  bg: 'Bulgarian',
  sk: 'Slovak',
  sl: 'Slovenian',
  lt: 'Lithuanian',
  lv: 'Latvian',
  et: 'Estonian',
  id: 'Indonesian',
  ms: 'Malay',
  tl: 'Tagalog',
  ta: 'Tamil',
  te: 'Telugu',
  kn: 'Kannada',
  ml: 'Malayalam',
  bn: 'Bengali',
  ne: 'Nepali',
  fa: 'Persian',
  bs: 'Bosnian',
  mk: 'Macedonian',
  is: 'Icelandic',
  sq: 'Albanian',
  be: 'Belarusian',
  ca: 'Catalan',
  // Common MediaInfo full names that differ from our enum
  'chinese (simplified)': 'Chinese Simplified',
  'chinese (traditional)': 'Chinese Traditional',
  'chinese simplified': 'Chinese Simplified',
  'chinese traditional': 'Chinese Traditional',
}

function normalizeLanguage(lang: string): string | null {
  // Direct match against enum values (case-insensitive check)
  const match = [...languageValues].find((v) => v.toLowerCase() === lang.toLowerCase())
  if (match) return match

  // Check aliases
  const alias = languageAliases[lang.toLowerCase()]
  if (alias) return alias

  return null
}

export default class MediainfoConverter {
  convert(info: ParseResult, fillReleaseNameGroup: boolean) {
    // const source = this.extractSource(info)
    const videoCodec = this.extractVideoCodec(info)
    // const processing = this.extractProcessing(info, videoCodec)
    const videoResolution = this.extractVideoResolution(info) // '720p' | ['1', '2']
    const container = this.extractContainer(info, videoResolution)
    const subtitlesLanguages = this.extractSubtitleLanguages(info) // ['Chinese Simplified']
    const features = this.extractFeatures(info)
    const releaseGroup = fillReleaseNameGroup ? this.extractReleaseGroup(info['general']['complete name']) : ''
    const releaseName = fillReleaseNameGroup ? this.extractReleaseName(info['general']['complete name']) : ''
    const sanitizedMediainfo = this.sanitizeMediainfo(info['originalMediainfo'])
    const audioCodec = this.extractAudioCodec(info)
    const audioChannels = this.extractAudioChannels(info)
    // const duration = this.extractDuration(info)
    const audioLanguages = this.extractAudioLanguages(info)
    return {
      sanitizedMediainfo,
      // source,
      video_codec: videoCodec,
      // processing,
      video_resolution: videoResolution,
      container,
      subtitle_languages: subtitlesLanguages,
      features,
      release_name: releaseName,
      release_group: releaseGroup,
      audio_codec: audioCodec,
      audio_channels: audioChannels,
      // duration,
      audio_languages: audioLanguages,
    }
  }

  sanitizeMediainfo(mediaInfo: string): string {
    return mediaInfo.replace(/^(Complete name\s*:\s*)(.*(?:[/\\]))?([^/\\]*)$/m, (match, p1, p2, p3) => p1 + p3)
  }

  extractReleaseName(path: string): string {
    const filename = path.substring(Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\')) + 1)
    const lastDotIndex = filename.lastIndexOf('.')
    return lastDotIndex === -1 ? filename : filename.substring(0, lastDotIndex)
  }

  extractReleaseGroup(releaseName: string) {
    return releaseName.substring(releaseName.lastIndexOf('-') + 1, releaseName.lastIndexOf('.')).trim()
  }

  extractFeatures(info: ParseResult) {
    const options = new Set<string>()
    if (/remux/i.test(info['general']['complete name'])) {
      options.add('Remux')
    }
    for (const v of info['video']) {
      const hdrFormat = v['hdr format']
      // const bitDepth = v['bit depth']
      if (hdrFormat && hdrFormat.match(/Dolby Vision/)) {
        options.add(VIDEO_OPTION.DOLBYVISION)
      }
      if (hdrFormat && hdrFormat.match(/HDR10\+/)) {
        options.add(VIDEO_OPTION.HDR10PLUS)
      } else if (hdrFormat && hdrFormat.match(/HDR10/)) {
        options.add(VIDEO_OPTION.HDR10)
      } else if (hdrFormat && hdrFormat.match(/HDR/)) {
        options.add(VIDEO_OPTION.HDR)
      }
    }
    return Array.from(options)
  }

  // extractAudioOption(info: ParseResult) {
  //   const options = new Set()
  //   for (const a of info['audio']) {
  //     const channels = a['channel(s)']
  //     const commercialName = a['commercial name']
  //     const format = a['format']
  //     if (channels && channels.match(/6 channels/)) {
  //       options.add(AUDIO_OPTION.CHANNEL51)
  //     }
  //     if (channels && channels.match(/8 channels/)) {
  //       options.add(AUDIO_OPTION.CHANNEL71)
  //     }
  //     if (commercialName && commercialName.match(/Atmos/)) {
  //       options.add(AUDIO_OPTION.DOLBYATMOS)
  //     }
  //     if (format && format.match(/DTS XLL X/)) {
  //       options.add(AUDIO_OPTION.DTSX)
  //     }
  //   }
  //   return Array.from(options)
  // }

  // not needed as this is specified at the edition_group level
  // extractSource(info: ParseResult) {
  //   const name = info['general']['complete name']
  //   return /bdrip|blu-?ray|bluray/i.test(name)
  //     ? 'Blu-ray'
  //     : /web/i.test(name)
  //       ? 'WEB'
  //       : /dvdrip|ifo|vob/i.test(name)
  //         ? 'DVD'
  //         : /hdtv/i.test(name)
  //           ? 'HDTV'
  //           : /tv/i.test(name)
  //             ? 'TV'
  //             : /vhs/i.test(name)
  //               ? 'VHS'
  //               : /hddvd/i.test(name)
  //                 ? 'HD-DVD'
  //                 : ''
  // }

  extractContainer(info: ParseResult, _videoResolution: VideoResolution) {
    const completeName = info['general']['complete name'] || ''
    const lastDotIndex = completeName.lastIndexOf('.')
    if (lastDotIndex === -1) {
      return ''
    }
    return completeName.substring(lastDotIndex + 1).toLowerCase()
  }

  extractVideoCodec(info: ParseResult): string | null {
    // V_MPEGH/ISO/HEVC is H265 ?
    const completeName = info['general']['complete name']
    const video = info['video'][0]
    // const encodingSettings = video['encoding settings']
    const format = video['format']
    const videoCodecId = video['codec id']
    const codec =
      format === 'AVC'
        ? 'h264'
        : format.includes('HEVC')
          ? 'h265'
          : format.includes('H265')
            ? 'h265'
            : format === 'MPEG-4 Visual'
              ? videoCodecId === 'XVID'
                ? 'XviD'
                : 'DivX'
              : format === 'RealVideo 4' || videoCodecId === 'RV40'
                ? 'RV40'
                : format === 'VC-1'
                  ? 'vc-1'
                  : format === 'VP9'
                    ? 'vp9'
                    : format === 'MPEG Video' && video['format version']?.includes('1')
                      ? 'mpeg1'
                      : format === 'MPEG Video'
                        ? 'mpeg2'
                        : /dvd5/i.test(completeName)
                          ? 'DVD5'
                          : /dvd9/i.test(completeName)
                            ? 'DVD9'
                            : null
    if (codec && !videoCodecValues.has(codec)) {
      return null
    }
    return codec
  }

  // extractProcessing(info: ParseResult, videoCodec: string) {
  //   const completeName = info['general']['complete name']
  //   return /remux/i.test(completeName)
  //     ? 'Remux'
  //     : ['x264', 'x265'].includes(videoCodec)
  //       ? 'Encode'
  //       : ['H.264', 'H.265'].includes(videoCodec)
  //         ? 'Untouched'
  //         : ''
  // }

  extractVideoResolution(info: ParseResult): string | [string, string] {
    const completeName = info['general']['complete name']
    const video = info['video'][0]
    const standard = video['standard']
    const scanType = video['scan type']

    const width = Number(video.width && (video.width.match(/[0-9 ]+/)?.[0].replace(/ /g, '') ?? ''))
    const height = Number(video.height && (video.height.match(/[0-9 ]+/)?.[0].replace(/ /g, '') ?? ''))

    // 1920x567 -> 1080p
    let videoResolution: string | [string, string] =
      /2160p/i.test(completeName) || width === 3840
        ? '2160p'
        : /1080i/i.test(completeName) || ((width === 1920 || (Number(width) < 1920 && height === 1080)) && (scanType === 'Interlaced' || scanType === 'MBAFF'))
          ? '1080i'
          : /1080p/i.test(completeName) || width === 1920 || (width < 1920 && height === 1080)
            ? '1080p'
            : /720p/i.test(completeName) || width === 1280 || (width < 1280 && height === 720)
              ? '720p'
              : width === 1024
                ? '576p'
                : standard === 'NTSC'
                  ? 'NTSC'
                  : width === 854 || height === 480
                    ? '480p'
                    : height === 360
                      ? '360p'
                      : standard === 'PAL'
                        ? 'PAL'
                        : 'Other'

    if (typeof videoResolution === 'string' && !videoResolutionValues.has(videoResolution)) {
      videoResolution = 'Other'
    }

    if (videoResolution === 'Other' && width && height) {
      videoResolution = [video.width, video.height] as [string, string]
    }

    return videoResolution
  }

  extractSubtitleLanguages(info: ParseResult) {
    const texts = info['text']
    const subtitleLanguages = []
    for (const text of texts) {
      let language = text['language'] || text['title']
      if (!language) {
        continue
      }
      let extra = ''
      if (language.match(/chinese|mandarin/i)) {
        language = 'Chinese'
        const title = compact([text['language'], text['title']]).join('\n')
        extra = title.match(/traditional|繁|cht/i) ? ' Traditional' : title.match(/simplified|简|chs/i) ? ' Simplified' : ' Simplified'
        subtitleLanguages.push(`${language}${extra}`)
      } else {
        const normalized = normalizeLanguage(language)
        if (normalized) {
          subtitleLanguages.push(normalized)
        }
      }
    }
    return uniq(subtitleLanguages)
  }

  extractAudioCodec(info: ParseResult): string {
    const audio = info['audio'][0]
    if (!audio) return ''

    const format = audio['format'] || ''
    const commercialName = audio['commercial name'] || ''

    // TrueHD (includes TrueHD Atmos -> mapped to true-hd)
    const codec =
      commercialName.match(/Dolby TrueHD/i) || format.match(/TrueHD/i)
        ? 'true-hd'
        : format.match(/DTS/i)
          ? 'dts'
          : format.match(/AC-?3/i) || format.match(/E-AC-?3/i)
            ? 'ac3'
            : format.match(/AAC/i)
              ? 'aac'
              : format.match(/FLAC/i)
                ? 'flac'
                : format.match(/PCM|LPCM/i)
                  ? 'pcm'
                  : format.match(/MP3|MPEG Audio/i)
                    ? 'mp3'
                    : format.match(/MLP FBA/i)
                      ? 'true-hd'
                      : format.match(/Opus/i)
                        ? 'opus'
                        : format.match(/Cook/i)
                          ? 'cook'
                          : format.match(/MPEG Audio/i)
                            ? 'mp2'
                            : format.match(/DSD/i)
                              ? 'dsd'
                              : ''

    if (codec && !audioCodecValues.has(codec)) {
      return ''
    }
    return codec
  }

  extractAudioChannels(info: ParseResult): string {
    const audio = info['audio'][0]
    if (!audio) return ''

    const channels = audio['channel(s)'] || ''
    const channelMatch = channels.match(/(\d+)\s*channel/i)

    if (channelMatch) {
      const numChannels = parseInt(channelMatch[1], 10)
      const mapped =
        numChannels === 8
          ? '7.1'
          : numChannels === 6
            ? '5.1'
            : numChannels === 5
              ? '5.0'
              : numChannels === 3
                ? '2.1'
                : numChannels === 2
                  ? '2.0'
                  : numChannels === 1
                    ? '1.0'
                    : ''
      if (mapped && !audioChannelsValues.has(mapped)) {
        return ''
      }
      return mapped
    }

    return ''
  }

  // extractDuration(info: ParseResult): string {
  //   const duration = info['general']['duration'] || ''
  //   return duration
  // }

  extractAudioLanguages(info: ParseResult): string[] {
    const audioTracks = info['audio']
    const languages: string[] = []

    for (const audio of audioTracks) {
      const language = audio['language']
      if (language) {
        const normalized = normalizeLanguage(language)
        if (normalized) {
          languages.push(normalized)
        }
      }
    }

    return uniq(languages)
  }
}
