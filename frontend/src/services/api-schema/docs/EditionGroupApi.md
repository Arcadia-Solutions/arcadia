# EditionGroupApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createEditionGroup**](#createeditiongroup) | **POST** /api/edition-groups | |

# **createEditionGroup**
> EditionGroup createEditionGroup(userCreatedEditionGroup)


### Example

```typescript
import {
    EditionGroupApi,
    Configuration,
    UserCreatedEditionGroup
} from './api';

const configuration = new Configuration();
const apiInstance = new EditionGroupApi(configuration);

let userCreatedEditionGroup: UserCreatedEditionGroup; //

const { status, data } = await apiInstance.createEditionGroup(
    userCreatedEditionGroup
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedEditionGroup** | **UserCreatedEditionGroup**|  | |


### Return type

**EditionGroup**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the edition_group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

