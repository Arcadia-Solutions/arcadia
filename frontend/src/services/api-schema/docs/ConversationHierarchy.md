# ConversationHierarchy


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**createdAt** | **string** |  | [default to undefined]
**id** | **number** |  | [default to undefined]
**messages** | [**Array&lt;ConversationMessageHierarchy&gt;**](ConversationMessageHierarchy.md) |  | [default to undefined]
**receiver** | [**UserLiteAvatar**](UserLiteAvatar.md) |  | [default to undefined]
**receiverLastSeenAt** | **string** |  | [default to undefined]
**sender** | [**UserLiteAvatar**](UserLiteAvatar.md) |  | [default to undefined]
**senderLastSeenAt** | **string** |  | [default to undefined]
**subject** | **string** |  | [default to undefined]

## Example

```typescript
import { ConversationHierarchy } from './api';

const instance: ConversationHierarchy = {
    createdAt,
    id,
    messages,
    receiver,
    receiverLastSeenAt,
    sender,
    senderLastSeenAt,
    subject,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
