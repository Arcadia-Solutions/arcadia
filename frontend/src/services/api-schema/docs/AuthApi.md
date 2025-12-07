# AuthApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**login**](#login) | **POST** /api/auth/login | |
|[**refreshToken**](#refreshtoken) | **POST** /api/auth/refresh-token | |
|[**register**](#register) | **POST** /api/auth/register | |

# **login**
> LoginResponse login(login)


### Example

```typescript
import {
    AuthApi,
    Configuration,
    Login
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let login: Login; //

const { status, data } = await apiInstance.login(
    login
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **login** | **Login**|  | |


### Return type

**LoginResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully logged in |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refreshToken**
> LoginResponse refreshToken(refreshToken)


### Example

```typescript
import {
    AuthApi,
    Configuration,
    RefreshToken
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let refreshToken: RefreshToken; //

const { status, data } = await apiInstance.refreshToken(
    refreshToken
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **refreshToken** | **RefreshToken**|  | |


### Return type

**LoginResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully refreshed the token |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **register**
> User register(register)


### Example

```typescript
import {
    AuthApi,
    Configuration,
    Register
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthApi(configuration);

let register: Register; //

const { status, data } = await apiInstance.register(
    register
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **register** | **Register**|  | |


### Return type

**User**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully registered the user |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

