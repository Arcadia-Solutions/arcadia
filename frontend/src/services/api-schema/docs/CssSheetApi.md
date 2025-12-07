# CssSheetApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createCSSSheet**](#createcsssheet) | **POST** /api/css-sheets | |
|[**editCSSSheet**](#editcsssheet) | **PUT** /api/css-sheets | |
|[**getCSSSheetContent**](#getcsssheetcontent) | **GET** /css/{name}.css | |
|[**getCSSSheets**](#getcsssheets) | **GET** /api/css-sheets | |

# **createCSSSheet**
> CssSheet createCSSSheet(userCreatedCssSheet)


### Example

```typescript
import {
    CssSheetApi,
    Configuration,
    UserCreatedCssSheet
} from './api';

const configuration = new Configuration();
const apiInstance = new CssSheetApi(configuration);

let userCreatedCssSheet: UserCreatedCssSheet; //

const { status, data } = await apiInstance.createCSSSheet(
    userCreatedCssSheet
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedCssSheet** | **UserCreatedCssSheet**|  | |


### Return type

**CssSheet**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully created the CSS sheet |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editCSSSheet**
> CssSheet editCSSSheet(editedCssSheet)


### Example

```typescript
import {
    CssSheetApi,
    Configuration,
    EditedCssSheet
} from './api';

const configuration = new Configuration();
const apiInstance = new CssSheetApi(configuration);

let editedCssSheet: EditedCssSheet; //

const { status, data } = await apiInstance.editCSSSheet(
    editedCssSheet
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedCssSheet** | **EditedCssSheet**|  | |


### Return type

**CssSheet**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the CSS sheet |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCSSSheetContent**
> getCSSSheetContent()


### Example

```typescript
import {
    CssSheetApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CssSheetApi(configuration);

let name: string; // (default to undefined)

const { status, data } = await apiInstance.getCSSSheetContent(
    name
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/css


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully retrieved the CSS content |  -  |
|**404** | CSS sheet not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCSSSheets**
> CssSheetsEnriched getCSSSheets()


### Example

```typescript
import {
    CssSheetApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CssSheetApi(configuration);

const { status, data } = await apiInstance.getCSSSheets();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**CssSheetsEnriched**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the css sheets |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

