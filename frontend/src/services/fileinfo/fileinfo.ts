import MediainfoParser from './mediainfo/mediainfoParser'
import MediainfoConverter from './mediainfo/mediainfoConverter'
import { removeMediainfoTag } from './utils'

function preProcess(text: string) {
  const replaces = [
    { from: '\u2002', to: ' ' },
    { from: '\u200d', to: '' },
  ]
  for (const { from, to } of replaces) {
    text = text.replaceAll(from, to)
  }
  return text
}

function getType(text: string) {
  text = preProcess(text)
  return text.match(/Disc (Title|Label)\s*:/i) ? 'bdinfo' : text.match(/Complete name\s*:/i) ? 'mediainfo' : null
}

export const getFileInfo = (text: string, fillReleaseNameGroup: boolean) => {
  text = preProcess(text)
  text = removeMediainfoTag(text)
  const type = getType(text)
  switch (type) {
    case 'mediainfo': {
      const info = new MediainfoParser().parse(text)
      if (!info) {
        return
      }
      const fields = new MediainfoConverter().convert(info, fillReleaseNameGroup)
      return fields
    }
    case 'bdinfo': {
      return null
      //const info = new BdinfoParser().parse(text)
      //if (!info) {
      //  return
      //}
      //return new BdinfoConverter().convert(info)
      return null
    }
    default:
      console.error('mediainfo unknown type, no Disc Title/Label or Complete name')
      return null
  }
}
