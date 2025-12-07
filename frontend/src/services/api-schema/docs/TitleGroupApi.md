# TitleGroupApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createTitleGroup**](#createtitlegroup) | **POST** /api/title-groups | |
|[**createTitleGroupComment**](#createtitlegroupcomment) | **POST** /api/title-groups/comments | |
|[**editTitleGroup**](#edittitlegroup) | **PUT** /api/title-groups | |
|[**getTitleGroup**](#gettitlegroup) | **GET** /api/title-groups | |
|[**getTitleGroupInfoLite**](#gettitlegroupinfolite) | **GET** /api/title-groups/lite | |

# **createTitleGroup**
> TitleGroup createTitleGroup(userCreatedTitleGroup)


### Example

```typescript
import {
    TitleGroupApi,
    Configuration,
    UserCreatedTitleGroup
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupApi(configuration);

let userCreatedTitleGroup: UserCreatedTitleGroup; //

const { status, data } = await apiInstance.createTitleGroup(
    userCreatedTitleGroup
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTitleGroup** | **UserCreatedTitleGroup**|  | |


### Return type

**TitleGroup**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the title_group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTitleGroupComment**
> TitleGroupComment createTitleGroupComment(userCreatedTitleGroupComment)


### Example

```typescript
import {
    TitleGroupApi,
    Configuration,
    UserCreatedTitleGroupComment
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupApi(configuration);

let userCreatedTitleGroupComment: UserCreatedTitleGroupComment; //

const { status, data } = await apiInstance.createTitleGroupComment(
    userCreatedTitleGroupComment
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTitleGroupComment** | **UserCreatedTitleGroupComment**|  | |


### Return type

**TitleGroupComment**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully posted the comment |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editTitleGroup**
> TitleGroup editTitleGroup(editedTitleGroup)


### Example

```typescript
import {
    TitleGroupApi,
    Configuration,
    EditedTitleGroup
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupApi(configuration);

let editedTitleGroup: EditedTitleGroup; //

const { status, data } = await apiInstance.editTitleGroup(
    editedTitleGroup
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedTitleGroup** | **EditedTitleGroup**|  | |


### Return type

**TitleGroup**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the title group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTitleGroup**
> TitleGroupAndAssociatedData getTitleGroup()


### Example

```typescript
import {
    TitleGroupApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getTitleGroup(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**TitleGroupAndAssociatedData**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the title_group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTitleGroupInfoLite**
> TitleGroupLite getTitleGroupInfoLite()


### Example

```typescript
import {
    TitleGroupApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TitleGroupApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getTitleGroupInfoLite(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**TitleGroupLite**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the title_group (lite info) |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

