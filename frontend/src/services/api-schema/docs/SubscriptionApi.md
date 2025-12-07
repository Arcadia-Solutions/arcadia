# SubscriptionApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createForumThreadPostsSubscription**](#createforumthreadpostssubscription) | **POST** /api/subscriptions/forum-thread-posts | |
|[**createTitleGroupTorrentsSubscription**](#createtitlegrouptorrentssubscription) | **POST** /api/subscriptions/title-group-torrents | |
|[**removeForumThreadPostsSubscription**](#removeforumthreadpostssubscription) | **DELETE** /api/subscriptions/forum-thread-posts | |
|[**removeTitleGroupTorrentsSubscription**](#removetitlegrouptorrentssubscription) | **DELETE** /api/subscriptions/title-group-torrents | |

# **createForumThreadPostsSubscription**
> createForumThreadPostsSubscription()


### Example

```typescript
import {
    SubscriptionApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubscriptionApi(configuration);

let threadId: number; // (default to undefined)

const { status, data } = await apiInstance.createForumThreadPostsSubscription(
    threadId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **threadId** | [**number**] |  | defaults to undefined|


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
|**200** | Successfully subscribed to the item |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTitleGroupTorrentsSubscription**
> createTitleGroupTorrentsSubscription()


### Example

```typescript
import {
    SubscriptionApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubscriptionApi(configuration);

let titleGroupId: number; // (default to undefined)

const { status, data } = await apiInstance.createTitleGroupTorrentsSubscription(
    titleGroupId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **titleGroupId** | [**number**] |  | defaults to undefined|


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
|**200** | Successfully subscribed to the item |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeForumThreadPostsSubscription**
> removeForumThreadPostsSubscription()


### Example

```typescript
import {
    SubscriptionApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubscriptionApi(configuration);

let threadId: number; // (default to undefined)

const { status, data } = await apiInstance.removeForumThreadPostsSubscription(
    threadId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **threadId** | [**number**] |  | defaults to undefined|


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
|**200** | Successfully unsubscribed to the item |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeTitleGroupTorrentsSubscription**
> removeTitleGroupTorrentsSubscription()


### Example

```typescript
import {
    SubscriptionApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SubscriptionApi(configuration);

let titleGroupId: number; // (default to undefined)

const { status, data } = await apiInstance.removeTitleGroupTorrentsSubscription(
    titleGroupId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **titleGroupId** | [**number**] |  | defaults to undefined|


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
|**200** | Successfully unsubscribed to the item |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

