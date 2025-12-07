# TorrentRequestAndAssociatedData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affiliatedArtists** | [**Array&lt;AffiliatedArtistHierarchy&gt;**](AffiliatedArtistHierarchy.md) |  | [default to undefined]
**comments** | [**Array&lt;TorrentRequestCommentHierarchy&gt;**](TorrentRequestCommentHierarchy.md) |  | [default to undefined]
**series** | [**SeriesLite**](SeriesLite.md) |  | [default to undefined]
**titleGroup** | [**TitleGroup**](TitleGroup.md) |  | [default to undefined]
**torrentRequest** | [**TorrentRequest**](TorrentRequest.md) |  | [default to undefined]
**votes** | [**Array&lt;TorrentRequestVoteHierarchy&gt;**](TorrentRequestVoteHierarchy.md) |  | [default to undefined]

## Example

```typescript
import { TorrentRequestAndAssociatedData } from './api';

const instance: TorrentRequestAndAssociatedData = {
    affiliatedArtists,
    comments,
    series,
    titleGroup,
    torrentRequest,
    votes,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
