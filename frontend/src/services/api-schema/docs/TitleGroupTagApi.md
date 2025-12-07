# TitleGroupTagApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**applyTagToTitleGroup**](#applytagtotitlegroup) | **POST** /api/title-group-tags/apply | |
|[**createTitleGroupTag**](#createtitlegrouptag) | **POST** /api/title-group-tags | |
|[**deleteTitleGroupTag**](#deletetitlegrouptag) | **DELETE** /api/title-group-tags | |
|[**editTitleGroupTag**](#edittitlegrouptag) | **PUT** /api/title-group-tags | |
|[**removeTagFromTitleGroup**](#removetagfromtitlegroup) | **DELETE** /api/title-group-tags/remove | |

# **applyTagToTitleGroup**
> applyTagToTitleGroup(appliedTitleGroupTag)


### Example

```typescript
import {
    TitleGroupTagApi,
    Configuration,
    AppliedTitleGroupTag
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupTagApi(configuration);

let appliedTitleGroupTag: AppliedTitleGroupTag; //

const { status, data } = await apiInstance.applyTagToTitleGroup(
    appliedTitleGroupTag
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **appliedTitleGroupTag** | **AppliedTitleGroupTag**|  | |


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
|**200** | Successfully applied the tag to the title group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTitleGroupTag**
> TitleGroupTag createTitleGroupTag(userCreatedTitleGroupTag)


### Example

```typescript
import {
    TitleGroupTagApi,
    Configuration,
    UserCreatedTitleGroupTag
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupTagApi(configuration);

let userCreatedTitleGroupTag: UserCreatedTitleGroupTag; //

const { status, data } = await apiInstance.createTitleGroupTag(
    userCreatedTitleGroupTag
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTitleGroupTag** | **UserCreatedTitleGroupTag**|  | |


### Return type

**TitleGroupTag**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully created the title group tag |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTitleGroupTag**
> deleteTitleGroupTag(deleteTagRequest)


### Example

```typescript
import {
    TitleGroupTagApi,
    Configuration,
    DeleteTagRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupTagApi(configuration);

let deleteTagRequest: DeleteTagRequest; //

const { status, data } = await apiInstance.deleteTitleGroupTag(
    deleteTagRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **deleteTagRequest** | **DeleteTagRequest**|  | |


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
|**200** | Successfully deleted the title group tag |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editTitleGroupTag**
> TitleGroupTag editTitleGroupTag(editedTitleGroupTag)


### Example

```typescript
import {
    TitleGroupTagApi,
    Configuration,
    EditedTitleGroupTag
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupTagApi(configuration);

let editedTitleGroupTag: EditedTitleGroupTag; //

const { status, data } = await apiInstance.editTitleGroupTag(
    editedTitleGroupTag
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedTitleGroupTag** | **EditedTitleGroupTag**|  | |


### Return type

**TitleGroupTag**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the title group tag |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeTagFromTitleGroup**
> removeTagFromTitleGroup(removedTitleGroupTag)


### Example

```typescript
import {
    TitleGroupTagApi,
    Configuration,
    RemovedTitleGroupTag
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupTagApi(configuration);

let removedTitleGroupTag: RemovedTitleGroupTag; //

const { status, data } = await apiInstance.removeTagFromTitleGroup(
    removedTitleGroupTag
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **removedTitleGroupTag** | **RemovedTitleGroupTag**|  | |


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
|**200** | Successfully removed the tag from the title group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

