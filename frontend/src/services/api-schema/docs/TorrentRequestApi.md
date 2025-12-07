# TorrentRequestApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createTorrentRequest**](#createtorrentrequest) | **POST** /api/torrent-requests | |
|[**createTorrentRequestComment**](#createtorrentrequestcomment) | **POST** /api/torrent-requests/comment | |
|[**createTorrentRequestVote**](#createtorrentrequestvote) | **POST** /api/torrent-requests/vote | |
|[**fillTorrentRequest**](#filltorrentrequest) | **POST** /api/torrent-requests/fill | |
|[**getTorrentRequests**](#gettorrentrequests) | **GET** /api/torrent-requests | |

# **createTorrentRequest**
> TorrentRequest createTorrentRequest(userCreatedTorrentRequest)


### Example

```typescript
import {
    TorrentRequestApi,
    Configuration,
    UserCreatedTorrentRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentRequestApi(configuration);

let userCreatedTorrentRequest: UserCreatedTorrentRequest; //

const { status, data } = await apiInstance.createTorrentRequest(
    userCreatedTorrentRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTorrentRequest** | **UserCreatedTorrentRequest**|  | |


### Return type

**TorrentRequest**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the torrent_request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTorrentRequestComment**
> TorrentRequestComment createTorrentRequestComment(userCreatedTorrentRequestComment)


### Example

```typescript
import {
    TorrentRequestApi,
    Configuration,
    UserCreatedTorrentRequestComment
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentRequestApi(configuration);

let userCreatedTorrentRequestComment: UserCreatedTorrentRequestComment; //

const { status, data } = await apiInstance.createTorrentRequestComment(
    userCreatedTorrentRequestComment
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTorrentRequestComment** | **UserCreatedTorrentRequestComment**|  | |


### Return type

**TorrentRequestComment**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully commented on the torrent request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTorrentRequestVote**
> TorrentRequestVote createTorrentRequestVote(userCreatedTorrentRequestVote)


### Example

```typescript
import {
    TorrentRequestApi,
    Configuration,
    UserCreatedTorrentRequestVote
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentRequestApi(configuration);

let userCreatedTorrentRequestVote: UserCreatedTorrentRequestVote; //

const { status, data } = await apiInstance.createTorrentRequestVote(
    userCreatedTorrentRequestVote
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTorrentRequestVote** | **UserCreatedTorrentRequestVote**|  | |


### Return type

**TorrentRequestVote**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully voted on the torrent_request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fillTorrentRequest**
> fillTorrentRequest(torrentRequestFill)


### Example

```typescript
import {
    TorrentRequestApi,
    Configuration,
    TorrentRequestFill
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentRequestApi(configuration);

let torrentRequestFill: TorrentRequestFill; //

const { status, data } = await apiInstance.fillTorrentRequest(
    torrentRequestFill
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **torrentRequestFill** | **TorrentRequestFill**|  | |


### Return type

void (empty response body)

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully filled the torrent request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTorrentRequests**
> TorrentRequestAndAssociatedData getTorrentRequests()


### Example

```typescript
import {
    TorrentRequestApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentRequestApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getTorrentRequests(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**TorrentRequestAndAssociatedData**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the torrent request with associated data |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

