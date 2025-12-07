# Profile


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lastFiveSnatchedTorrents** | [**Array&lt;TitleGroupHierarchyLite&gt;**](TitleGroupHierarchyLite.md) |  | [default to undefined]
**lastFiveUploadedTorrents** | [**Array&lt;TitleGroupHierarchyLite&gt;**](TitleGroupHierarchyLite.md) |  | [default to undefined]
**peers** | [**Array&lt;Peer&gt;**](Peer.md) |  | [default to undefined]
**unreadConversationsAmount** | **number** |  | [default to undefined]
**unreadNotificationsAmountForumThreadPosts** | **number** |  | [default to undefined]
**user** | [**User**](User.md) |  | [default to undefined]
**userWarnings** | [**Array&lt;UserWarning&gt;**](UserWarning.md) |  | [default to undefined]

## Example

```typescript
import { Profile } from './api';

const instance: Profile = {
    lastFiveSnatchedTorrents,
    lastFiveUploadedTorrents,
    peers,
    unreadConversationsAmount,
    unreadNotificationsAmountForumThreadPosts,
    user,
    userWarnings,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
