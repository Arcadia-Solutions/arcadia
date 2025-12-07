# InvitationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createInvitation**](#createinvitation) | **POST** /api/invitations | |

# **createInvitation**
> Invitation createInvitation(sentInvitation)


### Example

```typescript
import {
    InvitationApi,
    Configuration,
    SentInvitation
} from './api';

const configuration = new Configuration();
const apiInstance = new InvitationApi(configuration);

let sentInvitation: SentInvitation; //

const { status, data } = await apiInstance.createInvitation(
    sentInvitation
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **sentInvitation** | **SentInvitation**|  | |


### Return type

**Invitation**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully sent the invitation |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

