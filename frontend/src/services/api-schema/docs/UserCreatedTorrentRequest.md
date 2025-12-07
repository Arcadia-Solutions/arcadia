# UserCreatedTorrentRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audioBitrateSampling** | [**Array&lt;AudioBitrateSampling&gt;**](AudioBitrateSampling.md) |  | [default to undefined]
**audioChannels** | [**Array&lt;AudioChannels&gt;**](AudioChannels.md) |  | [default to undefined]
**audioCodec** | [**Array&lt;AudioCodec&gt;**](AudioCodec.md) |  | [default to undefined]
**container** | **Array&lt;string&gt;** |  | [default to undefined]
**description** | **string** |  | [optional] [default to undefined]
**editionName** | **string** |  | [optional] [default to undefined]
**features** | [**Array&lt;Features&gt;**](Features.md) |  | [default to undefined]
**initialVote** | [**UserCreatedTorrentRequestVote**](UserCreatedTorrentRequestVote.md) |  | [default to undefined]
**languages** | [**Array&lt;Language&gt;**](Language.md) |  | [default to undefined]
**releaseGroup** | **string** |  | [optional] [default to undefined]
**source** | [**Array&lt;Source&gt;**](Source.md) |  | [default to undefined]
**subtitleLanguages** | [**Array&lt;Language&gt;**](Language.md) |  | [default to undefined]
**titleGroupId** | **number** |  | [default to undefined]
**videoCodec** | [**Array&lt;VideoCodec&gt;**](VideoCodec.md) |  | [default to undefined]
**videoResolution** | [**Array&lt;VideoResolution&gt;**](VideoResolution.md) |  | [default to undefined]
**videoResolutionOtherX** | **number** |  | [optional] [default to undefined]
**videoResolutionOtherY** | **number** |  | [optional] [default to undefined]

## Example

```typescript
import { UserCreatedTorrentRequest } from './api';

const instance: UserCreatedTorrentRequest = {
    audioBitrateSampling,
    audioChannels,
    audioCodec,
    container,
    description,
    editionName,
    features,
    initialVote,
    languages,
    releaseGroup,
    source,
    subtitleLanguages,
    titleGroupId,
    videoCodec,
    videoResolution,
    videoResolutionOtherX,
    videoResolutionOtherY,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
