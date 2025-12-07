# GiftApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createGift**](#creategift) | **POST** /api/gifts | |

# **createGift**
> Gift createGift(userCreatedGift)


### Example

```typescript
import {
    GiftApi,
    Configuration,
    UserCreatedGift
} from './api';

const configuration = new Configuration();
const apiInstance = new GiftApi(configuration);

let userCreatedGift: UserCreatedGift; //

const { status, data } = await apiInstance.createGift(
    userCreatedGift
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedGift** | **UserCreatedGift**|  | |


### Return type

**Gift**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully sent the gift |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

