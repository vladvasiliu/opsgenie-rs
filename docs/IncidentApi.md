# \IncidentApi

All URIs are relative to *https://api.opsgenie.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_incident**](IncidentApi.md#close_incident) | **post** /v1/incidents/{identifier}/close | Close Incident
[**create_incident**](IncidentApi.md#create_incident) | **post** /v1/incidents/create | Create Incident
[**delete_incident**](IncidentApi.md#delete_incident) | **delete** /v1/incidents/{identifier} | Delete Incident
[**get_incident**](IncidentApi.md#get_incident) | **get** /v1/incidents/{identifier} | Get Incident
[**get_incident_request_status**](IncidentApi.md#get_incident_request_status) | **get** /v1/incidents/requests/{requestId} | Get Request Status of Incident
[**list_incidents**](IncidentApi.md#list_incidents) | **get** /v1/incidents/ | List incidents



## close_incident

> crate::models::SuccessResponse close_incident(identifier, identifier_type, close_incident_payload)
Close Incident

Closes incident with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of incident which could be incident id or tiny id | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id' or 'tiny. Default is id' |  |[default to id]
**close_incident_payload** | Option<[**CloseIncidentPayload**](CloseIncidentPayload.md)> | Request payload of closing incident action |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incident

> crate::models::SuccessResponse create_incident(create_incident_payload)
Create Incident

Creates a new incident

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_incident_payload** | [**CreateIncidentPayload**](CreateIncidentPayload.md) | Request payload of created incident | [required] |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_incident

> crate::models::SuccessResponse delete_incident(identifier, identifier_type)
Delete Incident

Deletes an incident using incident id or the tiny id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of incident which could be incident id or tiny id | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id' or 'tiny. Default is id' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incident

> crate::models::GetIncidentResponse get_incident(identifier, identifier_type)
Get Incident

Returns incident with given id, tiny id or alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of incident which could be incident id or tiny id | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id' or 'tiny. Default is id' |  |[default to id]

### Return type

[**crate::models::GetIncidentResponse**](GetIncidentResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incident_request_status

> crate::models::GetIncidentRequestStatusResponse get_incident_request_status(request_id)
Get Request Status of Incident

Used to track the status and incident details (if any) of the request whose identifier is given

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Universally unique identifier of the questioned request | [required] |

### Return type

[**crate::models::GetIncidentRequestStatusResponse**](GetIncidentRequestStatusResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incidents

> crate::models::ListIncidentsResponse list_incidents(query, offset, limit, sort, order)
List incidents

Return list of incidents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Search query to apply while filtering the incidents. | [required] |
**offset** | Option<**i32**> | Start index of the result set (to apply pagination). Minimum value (and also default value) is 0. |  |
**limit** | Option<**i32**> | Maximum number of items to provide in the result. Must be a positive integer value. Default value is 20 and maximum value is 100 |  |
**sort** | Option<**String**> | Name of the field that result set will be sorted by |  |[default to createdAt]
**order** | Option<**String**> | Sorting order of the result set |  |[default to desc]

### Return type

[**crate::models::ListIncidentsResponse**](ListIncidentsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

