# SeriesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createSeries**](#createseries) | **POST** /api/series | |
|[**getSeries**](#getseries) | **GET** /api/series | |

# **createSeries**
> Series createSeries(userCreatedSeries)


### Example

```typescript
import {
    SeriesApi,
    Configuration,
    UserCreatedSeries
} from './api';

const configuration = new Configuration();
const apiInstance = new SeriesApi(configuration);

let userCreatedSeries: UserCreatedSeries; //

const { status, data } = await apiInstance.createSeries(
    userCreatedSeries
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedSeries** | **UserCreatedSeries**|  | |


### Return type

**Series**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the series |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSeries**
> SeriesAndTitleGroupHierarchyLite getSeries()


### Example

```typescript
import {
    SeriesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SeriesApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getSeries(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**SeriesAndTitleGroupHierarchyLite**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the series |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

