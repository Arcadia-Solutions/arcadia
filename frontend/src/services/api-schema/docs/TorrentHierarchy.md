# TorrentHierarchy


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audioBitrate** | **number** |  | [optional] [default to undefined]
**audioBitrateSampling** | [**AudioBitrateSampling**](AudioBitrateSampling.md) |  | [optional] [default to undefined]
**audioChannels** | [**AudioChannels**](AudioChannels.md) |  | [optional] [default to undefined]
**audioCodec** | [**AudioCodec**](AudioCodec.md) |  | [optional] [default to undefined]
**container** | **string** |  | [default to undefined]
**createdAt** | **string** |  | [default to undefined]
**createdBy** | [**UserLite**](UserLite.md) |  | [optional] [default to undefined]
**createdById** | **number** |  | [optional] [default to undefined]
**description** | **string** |  | [optional] [default to undefined]
**downloadFactor** | **number** |  | [default to undefined]
**duration** | **number** |  | [optional] [default to undefined]
**editionGroupId** | **number** |  | [default to undefined]
**extras** | [**Array&lt;Extras&gt;**](Extras.md) |  | [default to undefined]
**features** | [**Array&lt;Features&gt;**](Features.md) |  | [default to undefined]
**fileAmountPerType** | **{ [key: string]: string; }** |  | [default to undefined]
**fileList** | **{ [key: string]: string; }** |  | [default to undefined]
**id** | **number** |  | [default to undefined]
**languages** | [**Array&lt;Language&gt;**](Language.md) |  | [default to undefined]
**leechers** | **number** |  | [default to undefined]
**mediainfo** | **string** |  | [optional] [default to undefined]
**releaseGroup** | **string** |  | [optional] [default to undefined]
**releaseName** | **string** |  | [optional] [default to undefined]
**reports** | [**Array&lt;TorrentReport&gt;**](TorrentReport.md) |  | [default to undefined]
**seeders** | **number** |  | [default to undefined]
**size** | **number** |  | [default to undefined]
**snatched** | **number** |  | [default to undefined]
**staffChecked** | **boolean** |  | [default to undefined]
**subtitleLanguages** | [**Array&lt;Language&gt;**](Language.md) |  | [default to undefined]
**timesCompleted** | **number** |  | [default to undefined]
**trumpable** | **string** |  | [optional] [default to undefined]
**updatedAt** | **string** |  | [default to undefined]
**uploadFactor** | **number** |  | [default to undefined]
**uploadedAsAnonymous** | **boolean** |  | [default to undefined]
**videoCodec** | [**VideoCodec**](VideoCodec.md) |  | [optional] [default to undefined]
**videoResolution** | [**VideoResolution**](VideoResolution.md) |  | [optional] [default to undefined]
**videoResolutionOtherX** | **number** |  | [optional] [default to undefined]
**videoResolutionOtherY** | **number** |  | [optional] [default to undefined]

## Example

```typescript
import { TorrentHierarchy } from './api';

const instance: TorrentHierarchy = {
    audioBitrate,
    audioBitrateSampling,
    audioChannels,
    audioCodec,
    container,
    createdAt,
    createdBy,
    createdById,
    description,
    downloadFactor,
    duration,
    editionGroupId,
    extras,
    features,
    fileAmountPerType,
    fileList,
    id,
    languages,
    leechers,
    mediainfo,
    releaseGroup,
    releaseName,
    reports,
    seeders,
    size,
    snatched,
    staffChecked,
    subtitleLanguages,
    timesCompleted,
    trumpable,
    updatedAt,
    uploadFactor,
    uploadedAsAnonymous,
    videoCodec,
    videoResolution,
    videoResolutionOtherX,
    videoResolutionOtherY,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
