# UserApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**editUser**](#edituser) | **PUT** /api/users | |
|[**getMe**](#getme) | **GET** /api/users/me | |
|[**getUserConversations**](#getuserconversations) | **GET** /api/users/conversations | |
|[**getUserSettings**](#getusersettings) | **GET** /api/users/settings | |
|[**getUsers**](#getusers) | **GET** /api/users | |
|[**updateUserSettings**](#updateusersettings) | **PUT** /api/users/settings | |
|[**warnUsers**](#warnusers) | **POST** /api/users/warnings | |

# **editUser**
> editUser(editedUser)


### Example

```typescript
import {
    UserApi,
    Configuration,
    EditedUser
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

let editedUser: EditedUser; //

const { status, data } = await apiInstance.editUser(
    editedUser
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedUser** | **EditedUser**|  | |


### Return type

void (empty response body)

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the user |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMe**
> Profile getMe()


### Example

```typescript
import {
    UserApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

const { status, data } = await apiInstance.getMe();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Profile**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the user\&#39;s profile |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserConversations**
> ConversationsOverview getUserConversations()


### Example

```typescript
import {
    UserApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

const { status, data } = await apiInstance.getUserConversations();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**ConversationsOverview**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Found the conversations and some of their metadata |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserSettings**
> UserSettings getUserSettings()


### Example

```typescript
import {
    UserApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

const { status, data } = await apiInstance.getUserSettings();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**UserSettings**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully retrieved user settings |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsers**
> PublicProfile getUsers()


### Example

```typescript
import {
    UserApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getUsers(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**PublicProfile**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the user\&#39;s profile |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUserSettings**
> updateUserSettings(userSettings)


### Example

```typescript
import {
    UserApi,
    Configuration,
    UserSettings
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

let userSettings: UserSettings; //

const { status, data } = await apiInstance.updateUserSettings(
    userSettings
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userSettings** | **UserSettings**|  | |


### Return type

void (empty response body)

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully updated user settings |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **warnUsers**
> UserWarning warnUsers(userCreatedUserWarning)


### Example

```typescript
import {
    UserApi,
    Configuration,
    UserCreatedUserWarning
} from './api';

const configuration = new Configuration();
const apiInstance = new UserApi(configuration);

let userCreatedUserWarning: UserCreatedUserWarning; //

const { status, data } = await apiInstance.warnUsers(
    userCreatedUserWarning
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedUserWarning** | **UserCreatedUserWarning**|  | |


### Return type

**UserWarning**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully warned the user |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

