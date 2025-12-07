# TorrentApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createTorrent**](#createtorrent) | **POST** /api/torrents | |
|[**createTorrentReport**](#createtorrentreport) | **POST** /api/torrents/reports | |
|[**deleteTorrent**](#deletetorrent) | **DELETE** /api/torrents | |
|[**downloadTorrentFile**](#downloadtorrentfile) | **GET** /api/torrents | |
|[**editTorrent**](#edittorrent) | **PUT** /api/torrents | |
|[**getRegisteredTorrents**](#getregisteredtorrents) | **GET** /api/torrents/registered | |
|[**getTopTorrent**](#gettoptorrent) | **GET** /api/torrents/top | |
|[**getUploadInformation**](#getuploadinformation) | **GET** /api/torrents/upload-info | |

# **createTorrent**
> Torrent createTorrent()


### Example

```typescript
import {
    TorrentApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let audioBitrate: number; // (default to undefined)
let audioBitrateSampling: AudioBitrateSampling; // (default to undefined)
let audioChannels: string; // (default to undefined)
let audioCodec: AudioCodec; // (default to undefined)
let container: string; // (default to undefined)
let description: string; // (default to undefined)
let duration: number; // (default to undefined)
let editionGroupId: number; // (default to undefined)
let extras: string; // (default to undefined)
let features: string; // (default to undefined)
let languages: string; // (default to undefined)
let mediainfo: string; // (default to undefined)
let releaseGroup: string; // (default to undefined)
let releaseName: string; // (default to undefined)
let subtitleLanguages: string; // (default to undefined)
let torrentFile: File; // (default to undefined)
let uploadedAsAnonymous: boolean; // (default to undefined)
let videoCodec: VideoCodec; // (default to undefined)
let videoResolution: VideoResolution; // (default to undefined)
let videoResolutionOtherX: number; // (default to undefined)
let videoResolutionOtherY: number; // (default to undefined)

const { status, data } = await apiInstance.createTorrent(
    audioBitrate,
    audioBitrateSampling,
    audioChannels,
    audioCodec,
    container,
    description,
    duration,
    editionGroupId,
    extras,
    features,
    languages,
    mediainfo,
    releaseGroup,
    releaseName,
    subtitleLanguages,
    torrentFile,
    uploadedAsAnonymous,
    videoCodec,
    videoResolution,
    videoResolutionOtherX,
    videoResolutionOtherY
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **audioBitrate** | [**number**] |  | defaults to undefined|
| **audioBitrateSampling** | **AudioBitrateSampling** |  | defaults to undefined|
| **audioChannels** | [**string**] |  | defaults to undefined|
| **audioCodec** | **AudioCodec** |  | defaults to undefined|
| **container** | [**string**] |  | defaults to undefined|
| **description** | [**string**] |  | defaults to undefined|
| **duration** | [**number**] |  | defaults to undefined|
| **editionGroupId** | [**number**] |  | defaults to undefined|
| **extras** | [**string**] |  | defaults to undefined|
| **features** | [**string**] |  | defaults to undefined|
| **languages** | [**string**] |  | defaults to undefined|
| **mediainfo** | [**string**] |  | defaults to undefined|
| **releaseGroup** | [**string**] |  | defaults to undefined|
| **releaseName** | [**string**] |  | defaults to undefined|
| **subtitleLanguages** | [**string**] |  | defaults to undefined|
| **torrentFile** | [**File**] |  | defaults to undefined|
| **uploadedAsAnonymous** | [**boolean**] |  | defaults to undefined|
| **videoCodec** | **VideoCodec** |  | defaults to undefined|
| **videoResolution** | **VideoResolution** |  | defaults to undefined|
| **videoResolutionOtherX** | [**number**] |  | defaults to undefined|
| **videoResolutionOtherY** | [**number**] |  | defaults to undefined|


### Return type

**Torrent**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Successfully uploaded the torrent |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTorrentReport**
> TorrentReport createTorrentReport(userCreatedTorrentReport)


### Example

```typescript
import {
    TorrentApi,
    Configuration,
    UserCreatedTorrentReport
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let userCreatedTorrentReport: UserCreatedTorrentReport; //

const { status, data } = await apiInstance.createTorrentReport(
    userCreatedTorrentReport
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **userCreatedTorrentReport** | **UserCreatedTorrentReport**|  | |


### Return type

**TorrentReport**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Torrent successfully reported |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTorrent**
> deleteTorrent(torrentToDelete)


### Example

```typescript
import {
    TorrentApi,
    Configuration,
    TorrentToDelete
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let torrentToDelete: TorrentToDelete; //

const { status, data } = await apiInstance.deleteTorrent(
    torrentToDelete
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **torrentToDelete** | **TorrentToDelete**|  | |


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
|**200** | Torrent deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **downloadTorrentFile**
> downloadTorrentFile()


### Example

```typescript
import {
    TorrentApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let id: number; // (default to undefined)

const { status, data } = await apiInstance.downloadTorrentFile(
    id
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **id** | [**number**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully downloaded the torrent file |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **editTorrent**
> Torrent editTorrent(editedTorrent)


### Example

```typescript
import {
    TorrentApi,
    Configuration,
    EditedTorrent
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let editedTorrent: EditedTorrent; //

const { status, data } = await apiInstance.editTorrent(
    editedTorrent
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **editedTorrent** | **EditedTorrent**|  | |


### Return type

**Torrent**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Successfully edited the torrent |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRegisteredTorrents**
> Array<TorrentMinimal> getRegisteredTorrents()


### Example

```typescript
import {
    TorrentApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

const { status, data } = await apiInstance.getRegisteredTorrents();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<TorrentMinimal>**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | All registered torrents |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTopTorrent**
> PaginatedResultsTorrentHierarchyLite getTopTorrent()


### Example

```typescript
import {
    TorrentApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

let period: string; // (default to undefined)
let amount: number; // (default to undefined)

const { status, data } = await apiInstance.getTopTorrent(
    period,
    amount
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **period** | [**string**] |  | defaults to undefined|
| **amount** | [**number**] |  | defaults to undefined|


### Return type

**PaginatedResultsTorrentHierarchyLite**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Top torrents found (and their title/edition group), sorted by amount of users who seeded at some point in time |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUploadInformation**
> UploadInformation getUploadInformation()


### Example

```typescript
import {
    TorrentApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new TorrentApi(configuration);

const { status, data } = await apiInstance.getUploadInformation();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**UploadInformation**

### Authorization

[http](../README.md#http)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Information related to uploading |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

