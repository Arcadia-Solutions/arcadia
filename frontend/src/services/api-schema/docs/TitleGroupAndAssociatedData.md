# TitleGroupAndAssociatedData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affiliatedArtists** | [**Array&lt;AffiliatedArtistHierarchy&gt;**](AffiliatedArtistHierarchy.md) |  | [default to undefined]
**affiliatedEntities** | [**Array&lt;AffiliatedEntityHierarchy&gt;**](AffiliatedEntityHierarchy.md) |  | [default to undefined]
**collages** | [**Array&lt;CollageSearchResult&gt;**](CollageSearchResult.md) |  | [default to undefined]
**editionGroups** | [**Array&lt;EditionGroupHierarchy&gt;**](EditionGroupHierarchy.md) |  | [default to undefined]
**inSameMasterGroup** | [**Array&lt;TitleGroupLite&gt;**](TitleGroupLite.md) |  | [default to undefined]
**isSubscribed** | **boolean** |  | [default to undefined]
**series** | [**SeriesLite**](SeriesLite.md) |  | [default to undefined]
**titleGroup** | [**TitleGroup**](TitleGroup.md) |  | [default to undefined]
**titleGroupComments** | [**Array&lt;TitleGroupCommentHierarchy&gt;**](TitleGroupCommentHierarchy.md) |  | [default to undefined]
**torrentRequests** | [**Array&lt;TorrentRequestHierarchyLite&gt;**](TorrentRequestHierarchyLite.md) |  | [default to undefined]

## Example

```typescript
import { TitleGroupAndAssociatedData } from './api';

const instance: TitleGroupAndAssociatedData = {
    affiliatedArtists,
    affiliatedEntities,
    collages,
    editionGroups,
    inSameMasterGroup,
    isSubscribed,
    series,
    titleGroup,
    titleGroupComments,
    torrentRequests,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
