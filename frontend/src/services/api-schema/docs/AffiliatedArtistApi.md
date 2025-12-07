# AffiliatedArtistApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createArtistAffiliation**](#createartistaffiliation) | **POST** /api/affiliated-artists | |
|[**deleteArtistAffiliation**](#deleteartistaffiliation) | **DELETE** /api/affiliated-artists | |

# **createArtistAffiliation**
> Array<AffiliatedArtistHierarchy> createArtistAffiliation(userCreatedAffiliatedArtist)


### Example

```typescript
import {
    AffiliatedArtistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AffiliatedArtistApi(configuration);

let userCreatedAffiliatedArtist: Array<UserCreatedAffiliatedArtist>; //

const { status, data } = await apiInstance.createArtistAffiliation(
    userCreatedAffiliatedArtist
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedAffiliatedArtist** | **Array<UserCreatedAffiliatedArtist>**|  | |


### Return type

**Array<AffiliatedArtistHierarchy>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the artist affiliations |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteArtistAffiliation**
> deleteArtistAffiliation()


### Example

```typescript
import {
    AffiliatedArtistApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AffiliatedArtistApi(configuration);

const { status, data } = await apiInstance.deleteArtistAffiliation();
```

### Parameters
This endpoint does not have any parameters.


### Return type

void (empty response body)

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully removed the artist affiliations |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

