# UserCreatedTitleGroup


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affiliatedArtists** | [**Array&lt;UserCreatedAffiliatedArtist&gt;**](UserCreatedAffiliatedArtist.md) |  | [default to undefined]
**category** | [**TitleGroupCategory**](TitleGroupCategory.md) |  | [optional] [default to undefined]
**contentType** | [**ContentType**](ContentType.md) |  | [default to undefined]
**countryFrom** | **string** |  | [optional] [default to undefined]
**covers** | **Array&lt;string&gt;** |  | [default to undefined]
**description** | **string** |  | [default to undefined]
**embeddedLinks** | **{ [key: string]: { [key: string]: string; }; }** |  | [default to undefined]
**externalLinks** | **Array&lt;string&gt;** |  | [default to undefined]
**masterGroupId** | **number** |  | [optional] [default to undefined]
**name** | **string** |  | [default to undefined]
**nameAliases** | **Array&lt;string&gt;** |  | [default to undefined]
**originalLanguage** | [**Language**](Language.md) |  | [optional] [default to undefined]
**originalReleaseDate** | **string** |  | [default to undefined]
**platform** | [**Platform**](Platform.md) |  | [optional] [default to undefined]
**screenshots** | **Array&lt;string&gt;** |  | [default to undefined]
**seriesId** | **number** |  | [optional] [default to undefined]
**tagline** | **string** |  | [optional] [default to undefined]
**tags** | **Array&lt;string&gt;** |  | [default to undefined]

## Example

```typescript
import { UserCreatedTitleGroup } from './api';

const instance: UserCreatedTitleGroup = {
    affiliatedArtists,
    category,
    contentType,
    countryFrom,
    covers,
    description,
    embeddedLinks,
    externalLinks,
    masterGroupId,
    name,
    nameAliases,
    originalLanguage,
    originalReleaseDate,
    platform,
    screenshots,
    seriesId,
    tagline,
    tags,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
