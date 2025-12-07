# ForumApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createForum**](#createforum) | **GET** /api/forum | |
|[**createForumPost**](#createforumpost) | **POST** /api/forum/post | |
|[**createForumThread**](#createforumthread) | **POST** /api/forum/thread | |
|[**getForimSubCategoryThread**](#getforimsubcategorythread) | **GET** /api/forum/sub-category | |
|[**getForumThread**](#getforumthread) | **GET** /api/forum/thread | |
|[**getForumThreadsPosts**](#getforumthreadsposts) | **GET** /api/forum/thread/posts | |

# **createForum**
> ForumOverview createForum()


### Example

```typescript
import {
    ForumApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

const { status, data } = await apiInstance.createForum();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**ForumOverview**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Returns an overview of the forum |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createForumPost**
> ForumPost createForumPost(userCreatedForumPost)


### Example

```typescript
import {
    ForumApi,
    Configuration,
    UserCreatedForumPost
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

let userCreatedForumPost: UserCreatedForumPost; //

const { status, data } = await apiInstance.createForumPost(
    userCreatedForumPost
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedForumPost** | **UserCreatedForumPost**|  | |


### Return type

**ForumPost**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the forum post |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createForumThread**
> ForumThread createForumThread(userCreatedForumThread)


### Example

```typescript
import {
    ForumApi,
    Configuration,
    UserCreatedForumThread
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

let userCreatedForumThread: UserCreatedForumThread; //

const { status, data } = await apiInstance.createForumThread(
    userCreatedForumThread
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedForumThread** | **UserCreatedForumThread**|  | |


### Return type

**ForumThread**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the forum thread |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getForimSubCategoryThread**
> ForumSubCategoryHierarchy getForimSubCategoryThread()


### Example

```typescript
import {
    ForumApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getForimSubCategoryThread(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**ForumSubCategoryHierarchy**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Returns the threads in the forum sub-category |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getForumThread**
> ForumThreadEnriched getForumThread()


### Example

```typescript
import {
    ForumApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getForumThread(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**ForumThreadEnriched**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Returns the thread\&#39;s information |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getForumThreadsPosts**
> PaginatedResultsForumPostHierarchy getForumThreadsPosts()


### Example

```typescript
import {
    ForumApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ForumApi(configuration);

let threadId: number; // (default to undefined)
let pageSize: number; // (default to undefined)
let page: number; // (optional) (default to undefined)
let postId: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.getForumThreadsPosts(
    threadId,
    pageSize,
    page,
    postId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **threadId** | [**number**] |  | defaults to undefined|
| **pageSize** | [**number**] |  | defaults to undefined|
| **page** | [**number**] |  | (optional) defaults to undefined|
| **postId** | [**number**] |  | (optional) defaults to undefined|


### Return type

**PaginatedResultsForumPostHierarchy**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Returns the thread\&#39;s posts |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

