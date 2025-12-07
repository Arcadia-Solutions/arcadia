# TorrentSearch


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artistId** | **number** |  | [optional] [default to undefined]
**collageId** | **number** |  | [optional] [default to undefined]
**orderByColumn** | [**TorrentSearchOrderByColumn**](TorrentSearchOrderByColumn.md) |  | [default to undefined]
**orderByDirection** | [**OrderByDirection**](OrderByDirection.md) |  | [default to undefined]
**page** | **number** |  | [default to undefined]
**pageSize** | **number** |  | [default to undefined]
**titleGroupIncludeEmptyGroups** | **boolean** |  | [default to undefined]
**titleGroupName** | **string** |  | [optional] [default to undefined]
**torrentCreatedById** | **number** |  | [optional] [default to undefined]
**torrentReported** | **boolean** |  | [optional] [default to undefined]
**torrentSnatchedById** | **number** |  | [optional] [default to undefined]
**torrentStaffChecked** | **boolean** |  | [optional] [default to undefined]

## Example

```typescript
import { TorrentSearch } from './api';

const instance: TorrentSearch = {
    artistId,
    collageId,
    orderByColumn,
    orderByDirection,
    page,
    pageSize,
    titleGroupIncludeEmptyGroups,
    titleGroupName,
    torrentCreatedById,
    torrentReported,
    torrentSnatchedById,
    torrentStaffChecked,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
