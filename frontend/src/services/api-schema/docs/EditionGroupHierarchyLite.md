# EditionGroupHierarchyLite


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additionalInformation** | **{ [key: string]: string; }** |  | [default to undefined]
**covers** | **Array&lt;string&gt;** |  | [default to undefined]
**distributor** | **string** |  | [optional] [default to undefined]
**id** | **number** |  | [default to undefined]
**name** | **string** |  | [optional] [default to undefined]
**releaseDate** | **string** |  | [default to undefined]
**source** | [**Source**](Source.md) |  | [optional] [default to undefined]
**titleGroupId** | **number** |  | [default to undefined]
**torrents** | [**Array&lt;TorrentHierarchyLite&gt;**](TorrentHierarchyLite.md) |  | [default to undefined]

## Example

```typescript
import { EditionGroupHierarchyLite } from './api';

const instance: EditionGroupHierarchyLite = {
    additionalInformation,
    covers,
    distributor,
    id,
    name,
    releaseDate,
    source,
    titleGroupId,
    torrents,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
