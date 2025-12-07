# ForumSubCategoryHierarchy


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | [**ForumCategoryLite**](ForumCategoryLite.md) |  | [default to undefined]
**forbiddenClasses** | **Array&lt;string&gt;** |  | [default to undefined]
**id** | **number** |  | [default to undefined]
**latestPostInThread** | [**ForumThreadPostLite**](ForumThreadPostLite.md) |  | [default to undefined]
**name** | **string** |  | [default to undefined]
**postsAmount** | **number** |  | [default to undefined]
**threads** | [**Array&lt;ForumThreadHierarchy&gt;**](ForumThreadHierarchy.md) |  | [optional] [default to undefined]
**threadsAmount** | **number** |  | [default to undefined]

## Example

```typescript
import { ForumSubCategoryHierarchy } from './api';

const instance: ForumSubCategoryHierarchy = {
    category,
    forbiddenClasses,
    id,
    latestPostInThread,
    name,
    postsAmount,
    threads,
    threadsAmount,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
