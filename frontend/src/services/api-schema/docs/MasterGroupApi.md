# MasterGroupApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createMasterGroup**](#createmastergroup) | **POST** /api/master-groups | |

# **createMasterGroup**
> MasterGroup createMasterGroup(userCreatedMasterGroup)


### Example

```typescript
import {
    MasterGroupApi,
    Configuration,
    UserCreatedMasterGroup
} from './api';

const configuration = new Configuration();
const apiInstance = new MasterGroupApi(configuration);

let userCreatedMasterGroup: UserCreatedMasterGroup; //

const { status, data } = await apiInstance.createMasterGroup(
    userCreatedMasterGroup
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedMasterGroup** | **UserCreatedMasterGroup**|  | |


### Return type

**MasterGroup**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the master group |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

