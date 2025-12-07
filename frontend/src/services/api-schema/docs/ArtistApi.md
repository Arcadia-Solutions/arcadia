# ArtistApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createArtist**](#createartist) | **POST** /api/artists | |
|[**editArtist**](#editartist) | **PUT** /api/artists | |
|[**getArtistPublications**](#getartistpublications) | **GET** /api/artists | |

# **createArtist**
> Array<Artist> createArtist(userCreatedArtist)


### Example

```typescript
import {
    ArtistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ArtistApi(configuration);

let userCreatedArtist: Array<UserCreatedArtist>; //

const { status, data } = await apiInstance.createArtist(
    userCreatedArtist
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedArtist** | **Array<UserCreatedArtist>**|  | |


### Return type

**Array<Artist>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully created the artists, returned in the same order as the one sent.             In the case of a db conflict (duplicate), the existing entry is returned (can be seen with the created_at attribute). |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editArtist**
> Artist editArtist(editedArtist)


### Example

```typescript
import {
    ArtistApi,
    Configuration,
    EditedArtist
} from './api';

const configuration = new Configuration();
const apiInstance = new ArtistApi(configuration);

let editedArtist: EditedArtist; //

const { status, data } = await apiInstance.editArtist(
    editedArtist
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedArtist** | **EditedArtist**|  | |


### Return type

**Artist**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the artist |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getArtistPublications**
> ArtistAndTitleGroupsLite getArtistPublications()


### Example

```typescript
import {
    ArtistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ArtistApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getArtistPublications(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**ArtistAndTitleGroupsLite**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the artist\&#39;s publications |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

