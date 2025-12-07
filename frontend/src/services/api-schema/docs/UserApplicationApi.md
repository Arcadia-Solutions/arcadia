# UserApplicationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createUserApplication**](#createuserapplication) | **POST** /api/auth/apply | |
|[**getUserApplications**](#getuserapplications) | **GET** /api/user-applications | |
|[**updateUserApplicationStatus**](#updateuserapplicationstatus) | **PUT** /api/user-applications | |

# **createUserApplication**
> UserApplication createUserApplication(userCreatedUserApplication)


### Example

```typescript
import {
    UserApplicationApi,
    Configuration,
    UserCreatedUserApplication
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApplicationApi(configuration);

let userCreatedUserApplication: UserCreatedUserApplication; //

const { status, data } = await apiInstance.createUserApplication(
    userCreatedUserApplication
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedUserApplication** | **UserCreatedUserApplication**|  | |


### Return type

**UserApplication**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully created user application |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserApplications**
> Array<UserApplication> getUserApplications()


### Example

```typescript
import {
    UserApplicationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApplicationApi(configuration);

let limit: number; //Maximum number of applications to return (default: 50) (optional) (default to undefined)
let page: number; //Page (default: 1) (optional) (default to undefined)
let status: string; //Filter by application status: \'pending\', \'accepted\', or \'rejected\' (optional) (default to undefined)
let checked: boolean; //Filter by checked status: true for checked (accepted/rejected), false for unchecked (pending) (optional) (default to undefined)

const { status, data } = await apiInstance.getUserApplications(
    limit,
    page,
    status,
    checked
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] | Maximum number of applications to return (default: 50) | (optional) defaults to undefined|
| **page** | [**number**] | Page (default: 1) | (optional) defaults to undefined|
| **status** | [**string**] | Filter by application status: \&#39;pending\&#39;, \&#39;accepted\&#39;, or \&#39;rejected\&#39; | (optional) defaults to undefined|
| **checked** | [**boolean**] | Filter by checked status: true for checked (accepted/rejected), false for unchecked (pending) | (optional) defaults to undefined|


### Return type

**Array<UserApplication>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully retrieved user applications |  -  |
|**400** | Bad Request - Invalid status parameter |  -  |
|**403** | Forbidden - Only staff members can view user applications |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUserApplicationStatus**
> UserApplication updateUserApplicationStatus(updateUserApplication)


### Example

```typescript
import {
    UserApplicationApi,
    Configuration,
    UpdateUserApplication
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApplicationApi(configuration);

let updateUserApplication: UpdateUserApplication; //

const { status, data } = await apiInstance.updateUserApplicationStatus(
    updateUserApplication
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateUserApplication** | **UpdateUserApplication**|  | |


### Return type

**UserApplication**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully updated user application status |  -  |
|**403** | Forbidden - Only staff members can update user applications |  -  |
|**404** | User application not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

