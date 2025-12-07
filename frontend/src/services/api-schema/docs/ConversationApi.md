# ConversationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createConversation**](#createconversation) | **POST** /api/conversations | |
|[**createConversationMessage**](#createconversationmessage) | **POST** /api/conversations/messages | |
|[**getConversations**](#getconversations) | **GET** /api/conversations | |

# **createConversation**
> Conversation createConversation(userCreatedConversation)


### Example

```typescript
import {
    ConversationApi,
    Configuration,
    UserCreatedConversation
} from './api';

const configuration = new Configuration();
const apiInstance = new ConversationApi(configuration);

let userCreatedConversation: UserCreatedConversation; //

const { status, data } = await apiInstance.createConversation(
    userCreatedConversation
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedConversation** | **UserCreatedConversation**|  | |


### Return type

**Conversation**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the conversation and first message |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createConversationMessage**
> ConversationMessage createConversationMessage(userCreatedConversationMessage)


### Example

```typescript
import {
    ConversationApi,
    Configuration,
    UserCreatedConversationMessage
} from './api';

const configuration = new Configuration();
const apiInstance = new ConversationApi(configuration);

let userCreatedConversationMessage: UserCreatedConversationMessage; //

const { status, data } = await apiInstance.createConversationMessage(
    userCreatedConversationMessage
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedConversationMessage** | **UserCreatedConversationMessage**|  | |


### Return type

**ConversationMessage**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully created the conversation\&#39;s message |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getConversations**
> ConversationHierarchy getConversations()


### Example

```typescript
import {
    ConversationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new ConversationApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.getConversations(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

**ConversationHierarchy**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Found the conversation and its messages |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

