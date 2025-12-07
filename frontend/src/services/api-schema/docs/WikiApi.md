# WikiApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createWikiArticle**](#createwikiarticle) | **POST** /api/wiki/articles | |
|[**editWikiArticle**](#editwikiarticle) | **PUT** /api/wiki/articles | |
|[**getWikiArticle**](#getwikiarticle) | **GET** /api/wiki/articles | |

# **createWikiArticle**
> WikiArticle createWikiArticle(userCreatedWikiArticle)


### Example

```typescript
import {
    WikiApi,
    Configuration,
    UserCreatedWikiArticle
} from './api';

const configuration = new Configuration();
const apiInstance = new WikiApi(configuration);

let userCreatedWikiArticle: UserCreatedWikiArticle; //

const { status, data } = await apiInstance.createWikiArticle(
    userCreatedWikiArticle
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedWikiArticle** | **UserCreatedWikiArticle**|  | |


### Return type

**WikiArticle**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the wiki article |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editWikiArticle**
> WikiArticle editWikiArticle(editedWikiArticle)


### Example

```typescript
import {
    WikiApi,
    Configuration,
    EditedWikiArticle
} from './api';

const configuration = new Configuration();
const apiInstance = new WikiApi(configuration);

let editedWikiArticle: EditedWikiArticle; //

const { status, data } = await apiInstance.editWikiArticle(
    editedWikiArticle
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedWikiArticle** | **EditedWikiArticle**|  | |


### Return type

**WikiArticle**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the wiki article |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getWikiArticle**
> WikiArticle getWikiArticle()


### Example

```typescript
import {
    WikiApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new WikiApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getWikiArticle(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**WikiArticle**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully found the wiki article |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

