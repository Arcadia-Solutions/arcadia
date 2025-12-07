# ExternalSourceApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getComicVineData**](#getcomicvinedata) | **GET** /api/external-sources/comic-vine | |
|[**getIsbnData**](#getisbndata) | **GET** /api/external-sources/isbn | |
|[**getMusicbranzData**](#getmusicbranzdata) | **GET** /api/external-sources/musicbrainz | |
|[**getTMDBData**](#gettmdbdata) | **GET** /api/external-sources/tmdb | |

# **getComicVineData**
> ExternalDBData getComicVineData()


### Example

```typescript
import {
    ExternalSourceApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ExternalSourceApi(configuration);

let url: string; // (default to undefined)

const { status, data } = await apiInstance.getComicVineData(
    url
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **url** | [**string**] |  | defaults to undefined|


### Return type

**ExternalDBData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getIsbnData**
> ExternalDBData getIsbnData()


### Example

```typescript
import {
    ExternalSourceApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ExternalSourceApi(configuration);

let isbn: string; // (default to undefined)

const { status, data } = await apiInstance.getIsbnData(
    isbn
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **isbn** | [**string**] |  | defaults to undefined|


### Return type

**ExternalDBData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMusicbranzData**
> ExternalDBData getMusicbranzData()


### Example

```typescript
import {
    ExternalSourceApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ExternalSourceApi(configuration);

let url: string; // (default to undefined)

const { status, data } = await apiInstance.getMusicbranzData(
    url
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **url** | [**string**] |  | defaults to undefined|


### Return type

**ExternalDBData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTMDBData**
> ExternalDBData getTMDBData()


### Example

```typescript
import {
    ExternalSourceApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ExternalSourceApi(configuration);

let url: string; // (default to undefined)

const { status, data } = await apiInstance.getTMDBData(
    url
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **url** | [**string**] |  | defaults to undefined|


### Return type

**ExternalDBData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

