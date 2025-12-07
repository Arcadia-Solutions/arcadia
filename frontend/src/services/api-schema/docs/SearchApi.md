# SearchApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**searchArtists**](#searchartists) | **GET** /api/search/artists/lite | |
|[**searchCollages**](#searchcollages) | **GET** /api/search/collages | |
|[**searchCollagesLite**](#searchcollageslite) | **GET** /api/search/collages/lite | |
|[**searchForum**](#searchforum) | **GET** /api/search/forum | |
|[**searchSeries**](#searchseries) | **GET** /api/search/series | |
|[**searchTitleGroupInfo**](#searchtitlegroupinfo) | **GET** /api/search/title-groups/lite | |
|[**searchTitleGroupTags**](#searchtitlegrouptags) | **GET** /api/search/title-group-tags/lite | |
|[**searchTorrentRequests**](#searchtorrentrequests) | **GET** /api/search/torrent-requests | |
|[**searchTorrents**](#searchtorrents) | **GET** /api/search/torrents/lite | |

# **searchArtists**
> Array<ArtistLite> searchArtists()

Case insensitive

### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let name: string; // (default to undefined)

const { status, data } = await apiInstance.searchArtists(
    name
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] |  | defaults to undefined|


### Return type

**Array<ArtistLite>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the artists and some data about them |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchCollages**
> PaginatedResultsCollageSearchResult searchCollages()

Case insensitive

### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let page: number; // (default to undefined)
let pageSize: number; // (default to undefined)
let name: string; // (optional) (default to undefined)
let tags: Array<string>; // (optional) (default to undefined)

const { status, data } = await apiInstance.searchCollages(
    page,
    pageSize,
    name,
    tags
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **page** | [**number**] |  | defaults to undefined|
| **pageSize** | [**number**] |  | defaults to undefined|
| **name** | [**string**] |  | (optional) defaults to undefined|
| **tags** | **Array&lt;string&gt;** |  | (optional) defaults to undefined|


### Return type

**PaginatedResultsCollageSearchResult**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the collages and some data about them |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchCollagesLite**
> Array<CollageLite> searchCollagesLite()

Case insensitive

### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let name: string; // (default to undefined)
let resultsAmount: number; // (default to undefined)

const { status, data } = await apiInstance.searchCollagesLite(
    name,
    resultsAmount
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] |  | defaults to undefined|
| **resultsAmount** | [**number**] |  | defaults to undefined|


### Return type

**Array<CollageLite>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the collages lite and some data about them |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchForum**
> PaginatedResultsForumSearchResult searchForum()

Case insensitive

### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let page: number; // (default to undefined)
let pageSize: number; // (default to undefined)
let threadName: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.searchForum(
    page,
    pageSize,
    threadName
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **page** | [**number**] |  | defaults to undefined|
| **pageSize** | [**number**] |  | defaults to undefined|
| **threadName** | [**string**] |  | (optional) defaults to undefined|


### Return type

**PaginatedResultsForumSearchResult**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the series and some data about them |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchSeries**
> SeriesSearchResponse searchSeries()

Case insensitive

### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let page: number; // (default to undefined)
let pageSize: number; // (default to undefined)
let name: string; // (optional) (default to undefined)
let tags: Array<string>; // (optional) (default to undefined)

const { status, data } = await apiInstance.searchSeries(
    page,
    pageSize,
    name,
    tags
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **page** | [**number**] |  | defaults to undefined|
| **pageSize** | [**number**] |  | defaults to undefined|
| **name** | [**string**] |  | (optional) defaults to undefined|
| **tags** | **Array&lt;string&gt;** |  | (optional) defaults to undefined|


### Return type

**SeriesSearchResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully got the series and some data about them |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchTitleGroupInfo**
> Array<TitleGroupLite> searchTitleGroupInfo()


### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let name: string; // (default to undefined)
let contentType: ContentType; // (optional) (default to undefined)

const { status, data } = await apiInstance.searchTitleGroupInfo(
    name,
    contentType
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] |  | defaults to undefined|
| **contentType** | **ContentType** |  | (optional) defaults to undefined|


### Return type

**Array<TitleGroupLite>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Returns title groups with their name containing the provided string, only the 5 first matches |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchTitleGroupTags**
> PaginatedResultsTitleGroupTagEnriched searchTitleGroupTags()


### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let name: string; //Search query (searches in tag name and synonyms) (default to undefined)
let page: number; //Page number (default to undefined)
let pageSize: number; //Results per page (default to undefined)

const { status, data } = await apiInstance.searchTitleGroupTags(
    name,
    page,
    pageSize
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **name** | [**string**] | Search query (searches in tag name and synonyms) | defaults to undefined|
| **page** | [**number**] | Page number | defaults to undefined|
| **pageSize** | [**number**] | Results per page | defaults to undefined|


### Return type

**PaginatedResultsTitleGroupTagEnriched**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List of matching tags with their names and synonyms |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchTorrentRequests**
> Array<TorrentRequestWithTitleGroupLite> searchTorrentRequests()


### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let titleGroupName: string; //Name of the title group to search for (optional) (default to undefined)
let tags: Array<string>; //Tags to filter title groups by (optional) (default to undefined)
let page: number; //Page number (default 1) (optional) (default to undefined)
let pageSize: number; //Results per page (default 50) (optional) (default to undefined)

const { status, data } = await apiInstance.searchTorrentRequests(
    titleGroupName,
    tags,
    page,
    pageSize
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **titleGroupName** | [**string**] | Name of the title group to search for | (optional) defaults to undefined|
| **tags** | **Array&lt;string&gt;** | Tags to filter title groups by | (optional) defaults to undefined|
| **page** | [**number**] | Page number (default 1) | (optional) defaults to undefined|
| **pageSize** | [**number**] | Results per page (default 50) | (optional) defaults to undefined|


### Return type

**Array<TorrentRequestWithTitleGroupLite>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List of torrent requests with associated title groups |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchTorrents**
> PaginatedResultsTitleGroupHierarchyLite searchTorrents()


### Example

```typescript
import {
    SearchApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new SearchApi(configuration);

let titleGroupIncludeEmptyGroups: boolean; // (default to undefined)
let page: number; // (default to undefined)
let pageSize: number; // (default to undefined)
let orderByColumn: TorrentSearchOrderByColumn; // (default to undefined)
let orderByDirection: OrderByDirection; // (default to undefined)
let titleGroupName: string; // (optional) (default to undefined)
let torrentReported: boolean; // (optional) (default to undefined)
let torrentStaffChecked: boolean; // (optional) (default to undefined)
let torrentCreatedById: number; // (optional) (default to undefined)
let torrentSnatchedById: number; // (optional) (default to undefined)
let artistId: number; // (optional) (default to undefined)
let collageId: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.searchTorrents(
    titleGroupIncludeEmptyGroups,
    page,
    pageSize,
    orderByColumn,
    orderByDirection,
    titleGroupName,
    torrentReported,
    torrentStaffChecked,
    torrentCreatedById,
    torrentSnatchedById,
    artistId,
    collageId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **titleGroupIncludeEmptyGroups** | [**boolean**] |  | defaults to undefined|
| **page** | [**number**] |  | defaults to undefined|
| **pageSize** | [**number**] |  | defaults to undefined|
| **orderByColumn** | **TorrentSearchOrderByColumn** |  | defaults to undefined|
| **orderByDirection** | **OrderByDirection** |  | defaults to undefined|
| **titleGroupName** | [**string**] |  | (optional) defaults to undefined|
| **torrentReported** | [**boolean**] |  | (optional) defaults to undefined|
| **torrentStaffChecked** | [**boolean**] |  | (optional) defaults to undefined|
| **torrentCreatedById** | [**number**] |  | (optional) defaults to undefined|
| **torrentSnatchedById** | [**number**] |  | (optional) defaults to undefined|
| **artistId** | [**number**] |  | (optional) defaults to undefined|
| **collageId** | [**number**] |  | (optional) defaults to undefined|


### Return type

**PaginatedResultsTitleGroupHierarchyLite**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Title groups and their torrents found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

