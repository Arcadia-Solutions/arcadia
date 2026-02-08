import {
  ContentType,
  AudioBitrateSampling,
  AudioChannels,
  AudioCodec,
  type CollageCategory,
  type EditionGroupInfoLite,
  type Extras,
  type Features,
  type Source,
  type Torrent,
  type TorrentRequest,
  VideoCodec,
  VideoResolution,
} from './api-schema'
import { OrderByDirection } from './api-schema'

export const timeAgo = (date: string) => {
  const diff = (Date.now() - new Date(date).getTime()) / 1000
  const absDiff = Math.abs(diff)
  const isFuture = diff < 0
  const format = (value: number, unit: string) => (isFuture ? `in ${value}${unit}` : `${value}${unit} ago`)
  return absDiff < 60
    ? format(Math.floor(absDiff), 's')
    : absDiff < 3600
      ? format(Math.floor(absDiff / 60), 'm')
      : absDiff < 86400
        ? format(Math.floor(absDiff / 3600), 'h')
        : format(Math.floor(absDiff / 86400), 'd')
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
export const getOrderByDirectionOptions = (t: (key: string) => string) => [
  { label: t('general.ascending'), value: OrderByDirection.Asc },
  { label: t('general.descending'), value: OrderByDirection.Desc },
]
export const getEditionGroupSlug = (editionGroup: EditionGroupInfoLite): string => {
  const attributes: (string | null)[] = []

  const formatReleaseDate = (date: string, onlyYearKnown: boolean): string => {
    if (onlyYearKnown) {
      return date.substring(0, 4)
    }
    return date
  }

  if (editionGroup.release_date) {
    let dateRange = ''
    if (editionGroup.additional_information?.date_from) {
      dateRange += formatReleaseDate(editionGroup.additional_information.date_from, editionGroup.release_date_only_year_known) + ' to '
    }
    dateRange += formatReleaseDate(editionGroup.release_date, editionGroup.release_date_only_year_known)

    let itemRange = ''
    if (editionGroup.additional_information?.first_item) {
      itemRange = ` (${editionGroup.additional_information.first_item} to ${editionGroup.additional_information.last_item})`
    }

    attributes.push(`${dateRange}${itemRange}`)
  } else {
    attributes.push(null)
  }
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
  // release_date isn't always mandatory
  return `${first ? first + ' - ' : ''}${rest}`
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
    'English',
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
    case 'video': {
      sources.push('Blu-Ray', 'DVD', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'movie': {
      sources.push('Blu-Ray', 'DVD', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'tv_show': {
      sources.push('Blu-Ray', 'DVD', 'HD-DVD', 'HD-TV', 'PDTV', 'VHS', 'TV', 'LaserDisc')
      break
    }
    case 'collection': {
      sources.push(
        'Blu-Ray',
        'DVD',
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
    case 'video': {
      extras.push('booklet', 'behind_the_scenes', 'deleted_scenes', 'featurette', 'trailer')
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
      return [
        ...commonRoles,
        'creator',
        'performer',
        'presenter',
        'contributor',
        'producer',
        'director',
        'cinematographer',
        'actor',
        'writer',
        'composer',
        'remixer',
      ]
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

export const getSelectableVideoCodecs = (): VideoCodec[] => {
  return Object.values(VideoCodec)
}

export const getSelectableVideoResolutions = (): VideoResolution[] => {
  return Object.values(VideoResolution)
}

export const getSelectableAudioCodecs = (): AudioCodec[] => {
  return Object.values(AudioCodec)
}

export const getSelectableAudioBitrateSamplings = (): AudioBitrateSampling[] => {
  return Object.values(AudioBitrateSampling)
}

export const getSelectableAudioChannels = (): AudioChannels[] => {
  return Object.values(AudioChannels)
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
export const isRouteProtected = (path: string) => {
  return ['/login', '/register', '/apply', '/home/index.html'].indexOf(path) < 0
}
export const isReleaseDateRequired = (contentType: ContentType): boolean => {
  const contentTypesRequiringReleaseDate: ContentType[] = ['movie', 'tv_show', 'music', 'podcast', 'software']
  return contentTypesRequiringReleaseDate.includes(contentType)
}

export const formatDateToLocalString = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export const formatBp = (rawValue: number, decimalPlaces: number, showDecimals = false): string => {
  const shifted = rawValue / Math.pow(10, decimalPlaces)
  const truncated = showDecimals ? shifted : Math.trunc(shifted)
  return truncated.toLocaleString(undefined, {
    minimumFractionDigits: showDecimals ? decimalPlaces : 0,
    maximumFractionDigits: showDecimals ? decimalPlaces : 0,
  })
}

export const rawToDisplayBp = (rawValue: number, decimalPlaces: number): number => {
  return rawValue / Math.pow(10, decimalPlaces)
}

export const displayToRawBp = (displayValue: number, decimalPlaces: number): number => {
  return Math.round(displayValue * Math.pow(10, decimalPlaces))
}

export const parseDateStringToLocal = (dateStr: string): Date | null => {
  const match = dateStr.match(/^(\d{4})-(\d{2})-(\d{2})/)
  if (!match) return null
  const [, year, month, day] = match.map(Number)
  return new Date(year, month - 1, day)
}
