import { expect, test, describe } from 'vitest'
import MediainfoParser from '../mediainfoParser'
import MediainfoConverter from '../mediainfoConverter'

// Real mediainfo from https://pastebin.com/raw/iW8L5eFn
const mediainfo = `General
Unique ID                                : 203316920412010605216290264579959007915 (0x98F56A1E20A21BBB90BDF57901D5AAAB)
Complete name                            : M:\\No Game No Life Zero 1080p Blu-ray Remux AVC DTS-HD MA 5.1 - XZVN.mkv
Format                                   : Matroska
Format version                           : Version 4 / Version 2
File size                                : 21.2 GiB
Duration                                 : 1 h 46 min
Overall bit rate                         : 28.5 Mb/s
Movie name                               : No Game No Life Zero 1080p Blu-ray Remux AVC DTS-HD MA 5.1 - XZVN
Encoded date                             : UTC 2018-02-25 19:24:42
Writing application                      : mkvmerge v20.0.0 ('I Am The Sun') 64-bit
Writing library                          : libebml v1.3.5 + libmatroska v1.4.8

Video
ID                                       : 1
Format                                   : AVC
Format/Info                              : Advanced Video Codec
Format profile                           : High@L4.1
Format settings                          : CABAC / 4 Ref Frames
Format settings, CABAC                   : Yes
Format settings, RefFrames               : 4 frames
Codec ID                                 : V_MPEG4/ISO/AVC
Duration                                 : 1 h 46 min
Bit rate mode                            : Constant
Bit rate                                 : 25.0 Mb/s / 25.0 Mb/s
Width                                    : 1 920 pixels
Height                                   : 1 080 pixels
Display aspect ratio                     : 16:9
Frame rate mode                          : Constant
Frame rate                               : 23.976 (24000/1001) FPS
Standard                                 : Component
Color space                              : YUV
Chroma subsampling                       : 4:2:0
Bit depth                                : 8 bits
Scan type                                : Progressive
Bits/(Pixel*Frame)                       : 0.503
Stream size                              : 18.4 GiB (87%)
Title                                    : MPEG-4 AVC Video / 25005 kbps / 1080p / 23.976 fps / 16:9 / High Profile 4.1
Writing library                          : x264 core 136
Encoding settings                        : cabac=1 / ref=4 / deblock=1:0:0 / analyse=0x3:0x113 / me=hex / subme=7 / psy=1 / psy_rd=1.00:0.00 / mixed_ref=1 / me_range=16 / chroma_me=1 / trellis=1 / 8x8dct=1 / cqm=2 / deadzone=21,11 / fast_pskip=1 / chroma_qp_offset=-2 / threads=6 / lookahead_threads=1 / sliced_threads=0 / slices=4 / nr=0 / decimate=1 / interlaced=0 / bluray_compat=1 / constrained_intra=0 / bframes=3 / b_pyramid=0 / b_adapt=1 / b_bias=0 / direct=1 / weightb=1 / open_gop=1 / weightp=1 / keyint=24 / keyint_min=1 / scenecut=40 / intra_refresh=0 / rc_lookahead=24 / rc=cbr / mbtree=1 / bitrate=25000 / ratetol=1.0 / qcomp=0.60 / qpmin=3 / qpmax=69 / qpstep=4 / vbv_maxrate=25000 / vbv_bufsize=25000 / nal_hrd=cbr / ip_ratio=1.40 / aq=1:1.00
Default                                  : Yes
Forced                                   : No
Color range                              : Limited
Color primaries                          : BT.709
Transfer characteristics                 : BT.709
Matrix coefficients                      : BT.709

Audio
ID                                       : 2
Format                                   : DTS
Format/Info                              : Digital Theater Systems
Format profile                           : MA / Core
Codec ID                                 : A_DTS
Duration                                 : 1 h 46 min
Bit rate mode                            : Variable / Constant
Bit rate                                 : 3 754 kb/s / 1 509 kb/s
Channel(s)                               : 6 channels
Channel positions                        : Front: L C R, Side: L R, LFE
Sampling rate                            : 48.0 kHz
Frame rate                               : 93.750 FPS (512 SPF)
Bit depth                                : 24 bits
Compression mode                         : Lossless / Lossy
Stream size                              : 2.79 GiB (13%)
Title                                    : Japanese / DTS-HD Master Audio / 5.1 / 48 kHz / 3754 kbps / 24-bit
Language                                 : Japanese
Default                                  : Yes
Forced                                   : No

Text
ID                                       : 3
Format                                   : ASS
Codec ID                                 : S_TEXT/ASS
Codec ID/Info                            : Advanced Sub Station Alpha
Duration                                 : 1 h 45 min
Bit rate                                 : 117 b/s
Count of elements                        : 1599
Compression mode                         : Lossless
Stream size                              : 90.2 KiB (0%)
Title                                    : Nekomimi-Subs & Winneon - animeisdead
Language                                 : English
Default                                  : Yes
Forced                                   : No

Menu
00:00:00.000                             : ja:00:00:00.000
00:05:01.468                             : ja:00:05:01.468
00:13:27.306                             : ja:00:13:27.306
00:33:18.997                             : ja:00:33:18.997
00:38:03.823                             : ja:00:38:03.823
00:44:23.286                             : ja:00:44:23.286
00:58:42.519                             : ja:00:58:42.519
01:04:49.636                             : ja:01:04:49.636
01:13:05.381                             : ja:01:13:05.381
01:19:51.537                             : ja:01:19:51.537
01:24:56.091                             : ja:01:24:56.091
01:33:25.850                             : ja:01:33:25.850
01:38:44.919                             : ja:01:38:44.919
01:41:11.565                             : ja:01:41:11.565
01:46:20.708                             : ja:01:46:20.708`

describe('mediainfoConverter', () => {
  const parser = new MediainfoParser()
  const converter = new MediainfoConverter()
  const parsed = parser.parse(mediainfo)
  const result = converter.convert(parsed)

  test('extracts video codec', () => {
    expect(result.video_codec).toBe('h264')
  })

  test('extracts video resolution', () => {
    expect(result.video_resolution).toBe('1080p')
  })

  test('extracts container', () => {
    expect(result.container).toBe('mkv')
  })

  test('extracts audio codec', () => {
    expect(result.audio_codec).toBe('dts')
  })

  test('extracts audio channels', () => {
    expect(result.audio_channels).toBe('5.1')
  })

  test('extracts audio languages', () => {
    expect(result.audio_languages).toEqual(['Japanese'])
  })

  test('extracts subtitle languages', () => {
    expect(result.subtitle_languages).toEqual(['English'])
  })

  test('extracts release name', () => {
    expect(result.release_name).toBe('No Game No Life Zero 1080p Blu-ray Remux AVC DTS-HD MA 5.1 - XZVN')
  })

  test('extracts release group', () => {
    expect(result.release_group).toBe('XZVN')
  })

  test('extracts Remux feature', () => {
    expect(result.features).toContain('Remux')
  })
})
