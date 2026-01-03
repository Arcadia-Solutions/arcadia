import { uniq, compact } from 'lodash-es'
import { VIDEO_OPTION } from '../utils'
import type { ParseResult } from './mediainfoParser'

type VideoResolution = string | [string, string]

export default class MediainfoConverter {
  convert(info: ParseResult) {
    // const source = this.extractSource(info)
    const videoCodec = this.extractVideoCodec(info)
    // const processing = this.extractProcessing(info, videoCodec)
    const videoResolution = this.extractVideoResolution(info) // '720p' | ['1', '2']
    const container = this.extractContainer(info, videoResolution)
    const subtitlesLanguages = this.extractSubtitleLanguages(info) // ['Chinese Simplified']
    const features = this.extractFeatures(info)
    const releaseGroup = this.extractReleaseGroup(info['general']['complete name'])
    const releaseName = this.extractReleaseName(info['general']['complete name'])
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

  extractContainer(info: ParseResult, videoResolution: VideoResolution) {
    const format = info['general']['format']
    if (!Array.isArray(videoResolution) && ['PAL', 'NTSC'].includes(videoResolution)) {
      return 'VOB IFO'
    }

    return /matroska/i.test(format)
      ? 'MKV'
      : /mpe?g-?4/i.test(format)
        ? 'MP4'
        : /mpe?g/i.test(format)
          ? 'MPG'
          : /avi/i.test(format)
            ? 'AVI'
            : /m2ts/i.test(format)
              ? 'm2ts'
              : /dvd/i.test(format)
                ? 'VOB IFO'
                : 'Other'
  }

  extractVideoCodec(info: ParseResult) {
    // V_MPEGH/ISO/HEVC is H265 ?
    const completeName = info['general']['complete name']
    const video = info['video'][0]
    // const encodingSettings = video['encoding settings']
    const format = video['format']
    const videoCodecId = video['codec id']
    return format === 'AVC'
      ? 'h264'
      : format.includes('HEVC')
        ? 'h265'
        : format.includes('H265')
          ? 'h265'
          : format === 'MPEG-4 Visual'
            ? videoCodecId === 'XVID'
              ? 'XviD'
              : 'DivX'
            : /dvd5/i.test(completeName)
              ? 'DVD5'
              : /dvd9/i.test(completeName)
                ? 'DVD9'
                : 'Other'
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
      }
      subtitleLanguages.push(`${language}${extra}`)
    }
    return uniq(subtitleLanguages)
  }

  extractAudioCodec(info: ParseResult): string {
    const audio = info['audio'][0]
    if (!audio) return ''

    const format = audio['format'] || ''
    const commercialName = audio['commercial name'] || ''

    // TrueHD (includes TrueHD Atmos -> mapped to true-hd)
    if (commercialName.match(/Dolby TrueHD/i) || format.match(/TrueHD/i)) {
      return 'true-hd'
    }
    // DTS (all variants: DTS-HD MA, DTS:X, DTS-HD HRA -> mapped to dts)
    if (format.match(/DTS/i)) {
      return 'dts'
    }
    // AC3 (includes EAC3 -> mapped to ac3)
    if (format.match(/AC-?3/i) || format.match(/E-AC-?3/i)) {
      return 'ac3'
    }
    if (format.match(/AAC/i)) {
      return 'aac'
    }
    if (format.match(/FLAC/i)) {
      return 'flac'
    }
    if (format.match(/PCM|LPCM/i)) {
      return 'pcm'
    }
    if (format.match(/MP3|MPEG Audio/i)) {
      return 'mp3'
    }
    if (format.match(/Opus/i)) {
      return 'opus'
    }

    return ''
  }

  extractAudioChannels(info: ParseResult): string {
    const audio = info['audio'][0]
    if (!audio) return ''

    const channels = audio['channel(s)'] || ''
    const channelMatch = channels.match(/(\d+)\s*channel/i)

    if (channelMatch) {
      const numChannels = parseInt(channelMatch[1], 10)
      if (numChannels === 8) return '7.1'
      if (numChannels === 6) return '5.1'
      if (numChannels === 5) return '5.0'
      if (numChannels === 2) return '2.0'
      if (numChannels === 1) return '1.0'
      return ''
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
        languages.push(language)
      }
    }

    return uniq(languages)
  }
}
