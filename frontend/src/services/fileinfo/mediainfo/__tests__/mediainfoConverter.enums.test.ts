import { expect, test, describe, vi } from 'vitest'

vi.mock('@/main', () => ({}))
vi.mock('@/services/api/api', () => ({ default: {} }))

import MediainfoParser from '../mediainfoParser'
import MediainfoConverter from '../mediainfoConverter'
import { VideoCodec, VideoResolution, AudioCodec, AudioChannels, Features } from '@/services/api-schema/api'

type Track = Record<string, string>
type Fixture = { general?: Track; video?: Track[]; audio?: Track[] }
type Extracted = ReturnType<MediainfoConverter['convert']>

const converter = new MediainfoConverter()

function buildMediainfo({ general = {}, video = [], audio = [] }: Fixture) {
  const section = (name: string, track: Track) =>
    `${name}\n` +
    Object.entries(track)
      .map(([key, value]) => `${key} : ${value}`)
      .join('\n')

  return [
    section('General', { 'Complete name': 'Sample.release.mkv', Format: 'Matroska', ...general }),
    ...video.map((track, index) => section('Video', { ID: String(index + 1), ...track })),
    ...audio.map((track, index) => section('Audio', { ID: String(index + 1), ...track })),
  ].join('\n\n')
}

function convert(fixture: Fixture): Extracted {
  return converter.convert(new MediainfoParser().parse(buildMediainfo(fixture)), false)
}

const asArray = (value: string | string[] | null) => (value === null ? [] : Array.isArray(value) ? value : [value])

type Extraction = {
  label: string
  values: readonly string[]
  pick: (result: Extracted) => string | string[] | null
  cases: { name: string; expected: string; fixture: Fixture }[]
  /** Enum values the extractor cannot produce, mapped to why */
  unsupported: Record<string, string>
}

function describeExtraction({ label, values, pick, cases, unsupported }: Extraction) {
  describe(`${label} extraction`, () => {
    test.each(cases)('extracts $expected from $name', ({ fixture, expected }) => {
      expect(asArray(pick(convert(fixture)))).toEqual(asArray(expected))
    })

    // Guards both ways: a codec added to the enum without a detection branch, and a
    // branch that regresses, both show up here rather than as a silently empty field.
    test(`covers every supported ${label} value`, () => {
      const expected = values.filter((value) => !(value in unsupported)).sort()
      const covered = [...new Set(cases.map(({ expected: value }) => value))].sort()
      expect(covered).toEqual(expected)
    })

    test(`only lists real ${label} values as unsupported`, () => {
      expect(Object.keys(unsupported).sort()).toEqual(values.filter((value) => value in unsupported).sort())
    })
  })
}

describeExtraction({
  label: 'VideoCodec',
  values: Object.values(VideoCodec),
  pick: (result) => result.video_codec,
  cases: [
    { name: 'AVC', expected: 'h264', fixture: { video: [{ Format: 'AVC' }] } },
    { name: 'HEVC', expected: 'h265', fixture: { video: [{ Format: 'HEVC' }] } },
    { name: 'H265', expected: 'h265', fixture: { video: [{ Format: 'H265' }] } },
    { name: 'MPEG-4 Visual with the XVID codec id', expected: 'XviD', fixture: { video: [{ Format: 'MPEG-4 Visual', 'Codec ID': 'XVID' }] } },
    { name: 'MPEG-4 Visual with a DivX codec id', expected: 'DivX', fixture: { video: [{ Format: 'MPEG-4 Visual', 'Codec ID': 'DIVX' }] } },
    { name: 'RealVideo 4', expected: 'RV40', fixture: { video: [{ Format: 'RealVideo 4' }] } },
    { name: 'Windows Media Video with the WMV2 codec id', expected: 'WMV2', fixture: { video: [{ Format: 'Windows Media Video', 'Codec ID': 'WMV2' }] } },
    { name: 'VC-1', expected: 'vc-1', fixture: { video: [{ Format: 'VC-1' }] } },
    { name: 'VP9', expected: 'vp9', fixture: { video: [{ Format: 'VP9' }] } },
    { name: 'VP6', expected: 'VP6', fixture: { video: [{ Format: 'VP6', 'Codec ID': 'V_VP6' }] } },
    { name: 'VP6F', expected: 'VP6', fixture: { video: [{ Format: 'VP6F', 'Codec ID': 'V_VP6F' }] } },
    { name: 'a VP6 track with no format', expected: 'VP6', fixture: { video: [{ 'Codec ID': 'V_VP6' }] } },
    { name: 'AV1 in Matroska', expected: 'av1', fixture: { video: [{ Format: 'AV1', 'Codec ID': 'V_AV1' }] } },
    { name: 'AV1 in MP4', expected: 'av1', fixture: { video: [{ Format: 'AV1', 'Codec ID': 'av01' }] } },
    { name: 'an AV1 track with no format', expected: 'av1', fixture: { video: [{ 'Codec ID': 'V_AV1' }] } },
    { name: 'MPEG Video version 1', expected: 'mpeg1', fixture: { video: [{ Format: 'MPEG Video', 'Format version': 'Version 1' }] } },
    { name: 'MPEG Video version 2', expected: 'mpeg2', fixture: { video: [{ Format: 'MPEG Video', 'Format version': 'Version 2' }] } },
    { name: 'a DVD5 release name', expected: 'DVD5', fixture: { general: { 'Complete name': 'Movie.DVD5.mkv' }, video: [{ Format: 'Unrecognized' }] } },
    { name: 'a DVD9 release name', expected: 'DVD9', fixture: { general: { 'Complete name': 'Movie.DVD9.mkv' }, video: [{ Format: 'Unrecognized' }] } },
  ],
  unsupported: {
    BD50: 'source tag, not a codec - no detection branch and no release name fallback like DVD5/DVD9',
    UHD100: 'source tag, not a codec - no detection branch and no release name fallback like DVD5/DVD9',
  },
})

describeExtraction({
  label: 'VideoResolution',
  values: Object.values(VideoResolution),
  pick: (result) => result.video_resolution,
  cases: [
    { name: 'a 3840 pixel wide track', expected: '2160p', fixture: { video: [{ Width: '3 840 pixels', Height: '2 160 pixels' }] } },
    {
      name: 'an interlaced 1920 pixel wide track',
      expected: '1080i',
      fixture: { video: [{ Width: '1 920 pixels', Height: '1 080 pixels', 'Scan type': 'Interlaced' }] },
    },
    { name: 'a progressive 1920 pixel wide track', expected: '1080p', fixture: { video: [{ Width: '1 920 pixels', Height: '1 080 pixels' }] } },
    { name: 'a 1280 pixel wide track', expected: '720p', fixture: { video: [{ Width: '1 280 pixels', Height: '720 pixels' }] } },
    { name: 'a 1024 pixel wide track', expected: '576p', fixture: { video: [{ Width: '1 024 pixels', Height: '576 pixels' }] } },
    { name: 'an NTSC track', expected: 'NTSC', fixture: { video: [{ Width: '720 pixels', Height: '480 pixels', Standard: 'NTSC' }] } },
    { name: 'a PAL track', expected: 'PAL', fixture: { video: [{ Width: '720 pixels', Height: '576 pixels', Standard: 'PAL' }] } },
    { name: 'a 854 pixel wide track', expected: '480p', fixture: { video: [{ Width: '854 pixels', Height: '480 pixels' }] } },
    { name: 'a 360 pixel high track', expected: '360p', fixture: { video: [{ Width: '640 pixels', Height: '360 pixels' }] } },
    { name: 'a track with no dimensions', expected: 'Other', fixture: { video: [{ Format: 'AVC' }] } },
  ],
  unsupported: {
    '480i': 'no detection branch produces it',
    '576i': 'no detection branch produces it',
    '1440p': 'no detection branch produces it, unrecognised sizes fall through to the custom dimensions pair',
    '4320p': 'no detection branch produces it, unrecognised sizes fall through to the custom dimensions pair',
  },
})

describeExtraction({
  label: 'AudioCodec',
  values: Object.values(AudioCodec),
  pick: (result) => result.audio_codec,
  cases: [
    { name: 'MPEG Audio layer 3', expected: 'mp3', fixture: { audio: [{ Format: 'MPEG Audio', 'Format profile': 'Layer 3' }] } },
    { name: 'AAC', expected: 'aac', fixture: { audio: [{ Format: 'AAC' }] } },
    { name: 'AC-3', expected: 'ac3', fixture: { audio: [{ Format: 'AC-3' }] } },
    { name: 'E-AC-3', expected: 'ac3', fixture: { audio: [{ Format: 'E-AC-3' }] } },
    { name: 'DTS', expected: 'dts', fixture: { audio: [{ Format: 'DTS' }] } },
    { name: 'DTS-HD Master Audio', expected: 'dts', fixture: { audio: [{ Format: 'DTS-HD Master Audio' }] } },
    { name: 'FLAC', expected: 'flac', fixture: { audio: [{ Format: 'FLAC' }] } },
    { name: 'PCM', expected: 'pcm', fixture: { audio: [{ Format: 'PCM' }] } },
    { name: 'LPCM', expected: 'pcm', fixture: { audio: [{ Format: 'LPCM' }] } },
    { name: 'MLP FBA', expected: 'true-hd', fixture: { audio: [{ Format: 'MLP FBA' }] } },
    { name: 'Dolby TrueHD Atmos', expected: 'true-hd', fixture: { audio: [{ Format: 'TrueHD', 'Commercial name': 'Dolby TrueHD Atmos' }] } },
    { name: 'Opus', expected: 'opus', fixture: { audio: [{ Format: 'Opus' }] } },
    { name: 'DSD', expected: 'dsd', fixture: { audio: [{ Format: 'DSD' }] } },
    { name: 'Cook', expected: 'cook', fixture: { audio: [{ Format: 'Cook' }] } },
    { name: 'WMA', expected: 'wma', fixture: { audio: [{ Format: 'WMA' }] } },
  ],
  unsupported: {
    mp2: 'unreachable - the mp3 branch matches the "MPEG Audio" format first, so layers 1 and 2 report as mp3',
  },
})

describeExtraction({
  label: 'AudioChannels',
  values: Object.values(AudioChannels),
  pick: (result) => result.audio_channels,
  cases: [
    { name: '1 channel', expected: '1.0', fixture: { audio: [{ 'Channel(s)': '1 channel' }] } },
    { name: '2 channels', expected: '2.0', fixture: { audio: [{ 'Channel(s)': '2 channels' }] } },
    { name: '3 channels', expected: '2.1', fixture: { audio: [{ 'Channel(s)': '3 channels' }] } },
    { name: '5 channels', expected: '5.0', fixture: { audio: [{ 'Channel(s)': '5 channels' }] } },
    { name: '6 channels', expected: '5.1', fixture: { audio: [{ 'Channel(s)': '6 channels' }] } },
    { name: '8 channels', expected: '7.1', fixture: { audio: [{ 'Channel(s)': '8 channels' }] } },
  ],
  unsupported: {},
})

describeExtraction({
  label: 'Features',
  values: Object.values(Features),
  pick: (result) => result.features,
  cases: [
    { name: 'an HDR track', expected: 'HDR', fixture: { video: [{ 'HDR format': 'HDR' }] } },
    { name: 'an HDR10 track', expected: 'HDR 10', fixture: { video: [{ 'HDR format': 'HDR10' }] } },
    { name: 'an HDR10+ track', expected: 'HDR 10+', fixture: { video: [{ 'HDR format': 'HDR10+' }] } },
    { name: 'a Dolby Vision track', expected: 'DV', fixture: { video: [{ 'HDR format': 'Dolby Vision, Version 1.0, dvhe.05.06' }] } },
    { name: 'a Remux release name', expected: 'Remux', fixture: { general: { 'Complete name': 'Movie.2160p.Blu-ray.Remux.mkv' } } },
  ],
  unsupported: {
    Commentary: 'no detection branch produces it',
    '3D': 'no detection branch produces it',
    OCR: 'no detection branch produces it',
    Cue: 'no detection branch produces it',
  },
})

test('reports the measured dimensions when no known resolution matches', () => {
  expect(convert({ video: [{ Width: '2 560 pixels', Height: '1 440 pixels' }] }).video_resolution).toEqual(['2 560 pixels', '1 440 pixels'])
})
