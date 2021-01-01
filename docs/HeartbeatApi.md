# \HeartbeatApi

All URIs are relative to *https://api.opsgenie.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_heartbeat**](HeartbeatApi.md#create_heartbeat) | **post** /v2/heartbeats | Create Heartbeat
[**delete_heartbeat**](HeartbeatApi.md#delete_heartbeat) | **delete** /v2/heartbeats/{name} | Delete Heartbeat
[**disable_heartbeat**](HeartbeatApi.md#disable_heartbeat) | **post** /v2/heartbeats/{name}/disable | Disable Heartbeat
[**enable_heartbeat**](HeartbeatApi.md#enable_heartbeat) | **post** /v2/heartbeats/{name}/enable | Enable Heartbeat
[**get_heartbeat**](HeartbeatApi.md#get_heartbeat) | **get** /v2/heartbeats/{name} | Get Heartbeat
[**list_heart_beats**](HeartbeatApi.md#list_heart_beats) | **get** /v2/heartbeats | List Heartbeats
[**ping**](HeartbeatApi.md#ping) | **get** /v2/heartbeats/{name}/ping | Ping Heartbeat
[**update_heartbeat**](HeartbeatApi.md#update_heartbeat) | **patch** /v2/heartbeats/{name} | Update Heartbeat (Partial)



## create_heartbeat

> crate::models::CreateHeartbeatResponse create_heartbeat(create_heartbeat_payload)
Create Heartbeat

Create a new heartbeat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_heartbeat_payload** | [**CreateHeartbeatPayload**](CreateHeartbeatPayload.md) | Request payload of created heartbeat | [required] |

### Return type

[**crate::models::CreateHeartbeatResponse**](CreateHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_heartbeat

> crate::models::SuccessResponse delete_heartbeat(name)
Delete Heartbeat

Delete heartbeat with given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_heartbeat

> crate::models::DisableHeartbeatResponse disable_heartbeat(name)
Disable Heartbeat

Disable heartbeat request with given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |

### Return type

[**crate::models::DisableHeartbeatResponse**](DisableHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_heartbeat

> crate::models::EnableHeartbeatResponse enable_heartbeat(name)
Enable Heartbeat

Enable heartbeat request with given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |

### Return type

[**crate::models::EnableHeartbeatResponse**](EnableHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_heartbeat

> crate::models::GetHeartbeatResponse get_heartbeat(name)
Get Heartbeat

Returns heartbeat with given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |

### Return type

[**crate::models::GetHeartbeatResponse**](GetHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_heart_beats

> crate::models::ListHeartbeatResponse list_heart_beats()
List Heartbeats

Returns list of Heartbeats

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListHeartbeatResponse**](ListHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping

> crate::models::SuccessResponse ping(name)
Ping Heartbeat

Ping Heartbeat for given heartbeat name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_heartbeat

> crate::models::UpdateHeartbeatResponse update_heartbeat(name, update_heartbeat_payload)
Update Heartbeat (Partial)

Update Heartbeatwith given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | [required] |
**update_heartbeat_payload** | Option<[**UpdateHeartbeatPayload**](UpdateHeartbeatPayload.md)> | Request payload of update heartbeat action |  |

### Return type

[**crate::models::UpdateHeartbeatResponse**](UpdateHeartbeatResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

