# NotificationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getNotificationsForForumThreadPosts**](#getnotificationsforforumthreadposts) | **POST** /api/notifications/forum-thread-posts | |

# **getNotificationsForForumThreadPosts**
> Array<NotificationForumThreadPost> getNotificationsForForumThreadPosts()


### Example

```typescript
import {
    NotificationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new NotificationApi(configuration);

let includeRead: boolean; // (default to undefined)

const { status, data } = await apiInstance.getNotificationsForForumThreadPosts(
    includeRead
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **includeRead** | [**boolean**] |  | defaults to undefined|


### Return type

**Array<NotificationForumThreadPost>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the notifications |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

