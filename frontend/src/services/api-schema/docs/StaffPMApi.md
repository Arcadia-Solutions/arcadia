# StaffPMApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createStaffPM**](#createstaffpm) | **POST** /api/staff-pms | |
|[**getStaffPM**](#getstaffpm) | **GET** /api/staff-pms/{id} | |
|[**listStaffPMs**](#liststaffpms) | **GET** /api/staff-pms | |
|[**replyToStaffPM**](#replytostaffpm) | **POST** /api/staff-pms/messages | |
|[**resolveStaffPM**](#resolvestaffpm) | **PUT** /api/staff-pms/{id}/resolve | |

# **createStaffPM**
> StaffPm createStaffPM(userCreatedStaffPm)


### Example

```typescript
import {
    StaffPMApi,
    Configuration,
    UserCreatedStaffPm
} from './api';

const configuration = new Configuration();
const apiInstance = new StaffPMApi(configuration);

let userCreatedStaffPm: UserCreatedStaffPm; //

const { status, data } = await apiInstance.createStaffPM(
    userCreatedStaffPm
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedStaffPm** | **UserCreatedStaffPm**|  | |


### Return type

**StaffPm**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Created staff PM conversation |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStaffPM**
> StaffPmHierarchy getStaffPM()


### Example

```typescript
import {
    StaffPMApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new StaffPMApi(configuration);

let id: number; //Staff PM id (default to undefined)

const { status, data } = await apiInstance.getStaffPM(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] | Staff PM id | defaults to undefined|


### Return type

**StaffPmHierarchy**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Staff PM conversation details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listStaffPMs**
> Array<StaffPmOverview> listStaffPMs()


### Example

```typescript
import {
    StaffPMApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new StaffPMApi(configuration);

const { status, data } = await apiInstance.listStaffPMs();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<StaffPmOverview>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List staff PM conversations |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replyToStaffPM**
> StaffPmMessage replyToStaffPM(userCreatedStaffPmMessage)


### Example

```typescript
import {
    StaffPMApi,
    Configuration,
    UserCreatedStaffPmMessage
} from './api';

const configuration = new Configuration();
const apiInstance = new StaffPMApi(configuration);

let userCreatedStaffPmMessage: UserCreatedStaffPmMessage; //

const { status, data } = await apiInstance.replyToStaffPM(
    userCreatedStaffPmMessage
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedStaffPmMessage** | **UserCreatedStaffPmMessage**|  | |


### Return type

**StaffPmMessage**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Created staff PM message |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **resolveStaffPM**
> StaffPm resolveStaffPM()


### Example

```typescript
import {
    StaffPMApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new StaffPMApi(configuration);

let id: number; //Staff PM id (default to undefined)

const { status, data } = await apiInstance.resolveStaffPM(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] | Staff PM id | defaults to undefined|


### Return type

**StaffPm**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Resolved staff PM |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

