# CollagesApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createACollage**](#createacollage) | **POST** /api/collages | |
|[**getACollage**](#getacollage) | **GET** /api/collages | |
|[**insertsEntriesIntoACollage**](#insertsentriesintoacollage) | **POST** /api/collages/entries | |

# **createACollage**
> Collage createACollage(userCreatedCollage)


### Example

```typescript
import {
    CollagesApi,
    Configuration,
    UserCreatedCollage
} from './api';

const configuration = new Configuration();
const apiInstance = new CollagesApi(configuration);

let userCreatedCollage: UserCreatedCollage; //

const { status, data } = await apiInstance.createACollage(
    userCreatedCollage
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedCollage** | **UserCreatedCollage**|  | |


### Return type

**Collage**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the collage |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getACollage**
> CollageAndAssociatedData getACollage()


### Example

```typescript
import {
    CollagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CollagesApi(configuration);

const { status, data } = await apiInstance.getACollage();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**CollageAndAssociatedData**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Collage information and its entries |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insertsEntriesIntoACollage**
> Array<CollageEntry> insertsEntriesIntoACollage(userCreatedCollageEntry)


### Example

```typescript
import {
    CollagesApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CollagesApi(configuration);

let userCreatedCollageEntry: Array<UserCreatedCollageEntry>; //

const { status, data } = await apiInstance.insertsEntriesIntoACollage(
    userCreatedCollageEntry
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedCollageEntry** | **Array<UserCreatedCollageEntry>**|  | |


### Return type

**Array<CollageEntry>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the collage entries |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

