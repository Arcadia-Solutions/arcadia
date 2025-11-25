import type { CollageCategory, CollageType } from './api/collageService'
import type { TorrentRequest } from './api/torrentRequestService'
import type { ContentType, EditionGroupInfoLite, Extras, Features, Source, Torrent } from './api/torrentService'

export const timeAgo = (date: string) => {
  const diff = (Date.now() - new Date(date).getTime()) / 1000
  return diff < 60
    ? `${Math.floor(diff)}s ago`
    : diff < 3600
      ? `${Math.floor(diff / 60)}m ago`
      : diff < 86400
        ? `${Math.floor(diff / 3600)}h ago`
        : `${Math.floor(diff / 86400)}d ago`
}
export const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  const time = date.toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
  })
  return `${date.getDate()} ${date.toLocaleString('default', { month: 'long' })} ${date.getFullYear()}, ${time}`
}
export const bytesToReadable = (bytes: number): string => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
}
export const getEditionGroupSlug = (editionGroup: EditionGroupInfoLite): string => {
  const attributes: string[] = []

  let dateRange = ''
  if (editionGroup.additional_information?.date_from) {
    dateRange += new Date(editionGroup.additional_information.date_from).toISOString().split('T')[0] + ' to '
  }
  dateRange += new Date(editionGroup.release_date).toISOString().split('T')[0]

  let itemRange = ''
  if (editionGroup.additional_information?.first_item) {
    itemRange = ` (${editionGroup.additional_information.first_item} to ${editionGroup.additional_information.last_item})`
  }

  attributes.push(`${dateRange}${itemRange}`)
  if (editionGroup.name) {
    attributes.push(editionGroup.name)
  }
  if (editionGroup.additional_information?.format) {
    attributes.push(editionGroup.additional_information.format)
  }

  if (editionGroup.additional_information?.label) {
    attributes.push(editionGroup.additional_information.label)
  }
  if (editionGroup.additional_information?.catalogue_number) {
    attributes.push(editionGroup.additional_information.catalogue_number)
  }
  if (editionGroup.additional_information?.isbn_13) {
    attributes.push(editionGroup.additional_information.isbn_13)
  }
  if (editionGroup.source) {
    attributes.push(editionGroup.source)
  }
  if (editionGroup.distributor) {
    attributes.push(editionGroup.distributor)
  }

  const first = attributes[0]
  const rest = attributes.slice(1).join(' / ')
  return `${first} - ${rest}`
}
export const getFeatures = (contentType: ContentType, format: string = '', source: Source | null = null): Features[] => {
  let features: Features[] = []
  if (source === 'Physical Book') {
    features = features.concat(['OCR'])
  }
  if ((contentType == 'book' && format === 'audiobook') || contentType == 'music') {
    features = features.concat(['Cue'])
  } else if (contentType == 'tv_show' || contentType == 'movie') {
    features = features.concat(['HDR', 'HDR 10', 'HDR 10+', 'DV', 'Commentary', 'Remux', '3D'])
  }
  return features
}
export const getLanguages = () => {
  return [
    'Albanian',
    'Arabic',
    'Belarusian',
    'Bengali',
    'Bosnian',
    'Bulgarian',
    'Cantonese',
    'Catalan',
    'Chinese',
    'Croatian',
    'Czech',
    'Danish',
    'Dutch',
    'English',
    'Estonian',
    'Finnish',
    'French',
    'German',
    'Greek',
    'Hebrew',
    'Hindi',
    'Hungarian',
    'Icelandic',
    'Indonesian',
    'Italian',
    'Japanese',
    'Kannada',
    'Korean',
    'Macedonian',
    'Malayalam',
    'Mandarin',
    'Nepali',
    'Norwegian',
    'Persian',
    'Polish',
    'Portuguese',
    'Romanian',
    'Russian',
    'Serbian',
    'Spanish',
    'Swedish',
    'Tamil',
    'Tagalog',
    'Telugu',
    'Thai',
    'Turkish',
    'Ukrainian',
    'Vietnamese',
    'Wolof',
    'Other',
  ]
}
export const getPlatforms = () => {
  return ['Linux', 'MacOS', 'Windows']
}
export const getSelectableContentTypes = (): ContentType[] => {
  return ['movie', 'video', 'tv_show', 'music', 'podcast', 'software', 'book', 'collection']
}
export const getCollageTypes = (): CollageType[] => {
  return ['TitleGroup' /* , 'Artist', 'Entity', 'MasterGroup'*/]
}
export const getCollageCategories = (): CollageCategory[] => {
  return ['External', 'Personal', 'Staff Picks', 'Theme']
}
export const getSources = (contentType: ContentType) => {
  const sources = ['Web']
  switch (contentType) {
    case 'book': {
      sources.push('Physical Book', 'CD')
      break
    }
    case 'music': {
      sources.push('Vinyl', 'Blu-Ray', 'CD', 'Soundboard', 'SACD', 'DAT', 'Cassette')
      break
    }
    case 'movie': {
      sources.push('Blu-Ray', 'DVD9', 'DVD5', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'tv_show': {
      sources.push('Blu-Ray', 'DVD9', 'DVD5', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'collection': {
      sources.push(
        'Blu-Ray',
        'DVD9',
        'DVD5',
        'HD-DVD',
        'HD-TV',
        'PDTV',
        'VHS',
        'TV',
        'LaserDisc',
        'Physical Book',
        'Vinyl',
        'CD',
        'Soundboard',
        'SACD',
        'DAT',
        'Cassette',
      )
      break
    }
  }
  sources.push('Mixed')
  return sources
}
export const getSelectableExtras = (contentType: ContentType) => {
  const extras: Extras[] = []
  switch (contentType) {
    case 'book': {
      extras.push('booklet')
      break
    }
    case 'music': {
      extras.push('booklet')
      break
    }
    case 'movie': {
      extras.push('behind_the_scenes', 'deleted_scenes', 'featurette', 'trailer')
      break
    }
    case 'tv_show': {
      extras.push('behind_the_scenes', 'deleted_scenes', 'trailer')
      break
    }
  }
  extras.push('other')
  return extras
}
export const getArtistRoles = (contentType: ContentType) => {
  const commonRoles = ['main', 'guest']

  switch (contentType) {
    case 'movie':
    case 'tv_show':
      return [...commonRoles, 'producer', 'director', 'cinematographer', 'actor', 'writer', 'composer']
    case 'video':
      return [...commonRoles, 'producer', 'director', 'cinematographer', 'actor', 'writer', 'composer', 'remixer']
    case 'music':
      return [...commonRoles, 'producer', 'composer', 'conductor', 'dj_compiler', 'remixer', 'arranger', 'writer']
    case 'podcast':
      return [...commonRoles, 'producer', 'writer', 'host']
    case 'book':
      return [...commonRoles, 'author', 'writer', 'illustrator', 'editor']
    case 'software':
      return [...commonRoles, 'developer', 'designer', 'producer', 'writer']
    case 'collection':
      return [...commonRoles, 'producer', 'director', 'composer', 'author', 'writer', 'editor']
    default:
      return commonRoles
  }
}
export const isValidUrl = (url: string) => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

export const getSelectableVideoCodecs = () => {
  return ['mpeg1', 'mpeg2', 'divX', 'DivX', 'h264', 'h265', 'vc-1', 'vp9', 'BD50', 'UHD100']
}

export const getSelectableVideoResolutions = () => {
  return ['Other', '480p', '480i', '576p', '576i', '720p', '1080p', '1080i', '1440p', '2160p', '4320p']
}

export const getSelectableAudioCodecs = () => {
  return ['aac', 'opus', 'mp3', 'mp2', 'aac', 'ac3', 'dts', 'flac', 'pcm', 'true-hd', 'dsd']
}

export const getSelectableAudioBitrateSamplings = () => {
  return [
    '64',
    '128',
    '192',
    '256',
    '320',
    'APS (VBR)',
    'V2 (VBR)',
    'V1 (VBR)',
    'V0 (VBR)',
    'APX (VBR)',
    'Lossless',
    '24bit Lossless',
    'DSD64',
    'DSD128',
    'DSD256',
    'DSD512',
    'Other',
  ]
}

export const getSelectableAudioChannels = () => {
  return ['1.0', '2.0', '2.1', '5.0', '5.1', '7.1']
}

export const getSelectableContainers = () => {
  return [
    'mkv',
    'mp4',
    'avi',
    'mov',
    'wmv',
    'flv',
    'webm',
    'm4v',
    '3gp',
    'ogv',
    'ts',
    'mts',
    'm2ts',
    'vob',
    'iso',
    'img',
    'bin',
    'cue',
    'flac',
    'mp3',
    'wav',
    'aac',
    'ogg',
    'm4a',
    'wma',
    'opus',
    'pdf',
    'epub',
    'mobi',
    'azw3',
    'cbz',
    'cbr',
    'zip',
    'rar',
    '7z',
    'tar',
    'gz',
    'bz2',
    'xz',
  ]
}

export const isAttributeUsed = (attribute: keyof Torrent | keyof TorrentRequest, contentType: ContentType): boolean => {
  switch (attribute) {
    case 'video_codec':
      return ['movie', 'tv_show', 'video', 'collection'].includes(contentType)
    case 'video_resolution':
      return ['movie', 'tv_show', 'video', 'collection'].includes(contentType)
    case 'video_resolution_other_x':
      return ['movie', 'tv_show', 'video', 'collection'].includes(contentType)
    case 'video_resolution_other_y':
      return ['movie', 'tv_show', 'video', 'collection'].includes(contentType)
    case 'audio_channels':
      return ['movie', 'tv_show', 'video', 'collection'].includes(contentType)
    case 'audio_bitrate_sampling':
      return ['movie', 'tv_show', 'video', 'music', 'podcast', 'collection'].includes(contentType)
    case 'audio_codec':
      return ['movie', 'tv_show', 'video', 'music', 'podcast', 'collection'].includes(contentType)
    case 'subtitle_languages':
      return ['movie', 'tv-show', 'video', 'collection'].includes(contentType)
    default:
      return true
  }
}
export const scrollToHash = () => {
  ;(function h() {
    const e = document.querySelector(location.hash)
    if (e) {
      e.scrollIntoView({ behavior: 'smooth' })
    } else {
      setTimeout(h, 100)
    }
  })()
}
export const getHostname = () => {
  return window.location.hostname
}
