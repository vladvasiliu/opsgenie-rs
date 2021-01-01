# \AlertApi

All URIs are relative to *https://api.opsgenie.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_alert**](AlertApi.md#acknowledge_alert) | **post** /v2/alerts/{identifier}/acknowledge | Acknowledge Alert
[**add_attachment**](AlertApi.md#add_attachment) | **post** /v2/alerts/{identifier}/attachments | Add Alert Attachment
[**add_details**](AlertApi.md#add_details) | **post** /v2/alerts/{identifier}/details | Add Details
[**add_note**](AlertApi.md#add_note) | **post** /v2/alerts/{identifier}/notes | Add Note
[**add_responder**](AlertApi.md#add_responder) | **post** /v2/alerts/{identifier}/responders | Add Responder
[**add_tags**](AlertApi.md#add_tags) | **post** /v2/alerts/{identifier}/tags | Add Tags
[**add_team**](AlertApi.md#add_team) | **post** /v2/alerts/{identifier}/teams | Add Team
[**assign_alert**](AlertApi.md#assign_alert) | **post** /v2/alerts/{identifier}/assign | Assign Alert
[**close_alert**](AlertApi.md#close_alert) | **post** /v2/alerts/{identifier}/close | Close Alert
[**count_alerts**](AlertApi.md#count_alerts) | **get** /v2/alerts/count | Count Alerts
[**create_alert**](AlertApi.md#create_alert) | **post** /v2/alerts | Create Alert
[**create_saved_searches**](AlertApi.md#create_saved_searches) | **post** /v2/alerts/saved-searches | Create Saved Search
[**delete_alert**](AlertApi.md#delete_alert) | **delete** /v2/alerts/{identifier} | Delete Alert
[**delete_saved_search**](AlertApi.md#delete_saved_search) | **delete** /v2/alerts/saved-searches/{identifier} | Delete Saved Search
[**escalate_alert**](AlertApi.md#escalate_alert) | **post** /v2/alerts/{identifier}/escalate | Escalate Alert
[**execute_custom_alert_action**](AlertApi.md#execute_custom_alert_action) | **post** /v2/alerts/{identifier}/actions/{actionName} | Custom Alert Action
[**get_alert**](AlertApi.md#get_alert) | **get** /v2/alerts/{identifier} | Get Alert
[**get_attachment**](AlertApi.md#get_attachment) | **get** /v2/alerts/{identifier}/attachments/{attachmentId} | Get Alert Attachment
[**get_request_status**](AlertApi.md#get_request_status) | **get** /v2/alerts/requests/{requestId} | Get Request Status of Alert
[**get_saved_search**](AlertApi.md#get_saved_search) | **get** /v2/alerts/saved-searches/{identifier} | Get Saved Search
[**list_alerts**](AlertApi.md#list_alerts) | **get** /v2/alerts | List Alerts
[**list_attachments**](AlertApi.md#list_attachments) | **get** /v2/alerts/{identifier}/attachments | List Alert Attachments
[**list_logs**](AlertApi.md#list_logs) | **get** /v2/alerts/{identifier}/logs | List Alert Logs
[**list_notes**](AlertApi.md#list_notes) | **get** /v2/alerts/{identifier}/notes | List Alert Notes
[**list_recipients**](AlertApi.md#list_recipients) | **get** /v2/alerts/{identifier}/recipients | List Alert Recipients
[**list_saved_searches**](AlertApi.md#list_saved_searches) | **get** /v2/alerts/saved-searches | Lists Saved Searches
[**remove_attachment**](AlertApi.md#remove_attachment) | **delete** /v2/alerts/{identifier}/attachments/{attachmentId} | Remove Alert Attachment
[**remove_details**](AlertApi.md#remove_details) | **delete** /v2/alerts/{identifier}/details | Remove Details
[**remove_tags**](AlertApi.md#remove_tags) | **delete** /v2/alerts/{identifier}/tags | Remove Tags
[**snooze_alert**](AlertApi.md#snooze_alert) | **post** /v2/alerts/{identifier}/snooze | Snooze Alert
[**un_acknowledge_alert**](AlertApi.md#un_acknowledge_alert) | **post** /v2/alerts/{identifier}/unacknowledge | UnAcknowledge Alert
[**update_alert_description**](AlertApi.md#update_alert_description) | **put** /v2/alerts/{identifier}/description | Update Alert Description
[**update_alert_message**](AlertApi.md#update_alert_message) | **put** /v2/alerts/{identifier}/message | Update Alert Message
[**update_alert_priority**](AlertApi.md#update_alert_priority) | **put** /v2/alerts/{identifier}/priority | Update Alert Priority
[**update_saved_search**](AlertApi.md#update_saved_search) | **patch** /v2/alerts/saved-searches/{identifier} | Update Saved Search



## acknowledge_alert

> crate::models::SuccessResponse acknowledge_alert(identifier, identifier_type, acknowledge_alert_payload)
Acknowledge Alert

Acknowledges alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**acknowledge_alert_payload** | Option<[**AcknowledgeAlertPayload**](AcknowledgeAlertPayload.md)> | Request payload of acknowledging alert action |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_attachment

> crate::models::SuccessResponse add_attachment(identifier, file, alert_identifier_type, user, index_file)
Add Alert Attachment

Add Alert Attachment to related alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**file** | **std::path::PathBuf** | Attachment file to be uploaded | [required] |
**alert_identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**user** | Option<**String**> | Display name of the request owner |  |
**index_file** | Option<**String**> | Name of html file which will be shown when attachment clicked on UI |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_details

> crate::models::SuccessResponse add_details(identifier, add_details_to_alert_payload, identifier_type)
Add Details

Add details to the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**add_details_to_alert_payload** | [**AddDetailsToAlertPayload**](AddDetailsToAlertPayload.md) | Request payload of adding alert details action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_note

> crate::models::SuccessResponse add_note(identifier, add_note_to_alert_payload, identifier_type)
Add Note

Adds note to alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**add_note_to_alert_payload** | [**AddNoteToAlertPayload**](AddNoteToAlertPayload.md) | Request payload of adding note to alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_responder

> crate::models::SuccessResponse add_responder(identifier, add_responder_to_alert_payload, identifier_type)
Add Responder

Add responder to alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**add_responder_to_alert_payload** | [**AddResponderToAlertPayload**](AddResponderToAlertPayload.md) | Request payload of adding responder to alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tags

> crate::models::SuccessResponse add_tags(identifier, add_tags_to_alert_payload, identifier_type)
Add Tags

Add tags to the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**add_tags_to_alert_payload** | [**AddTagsToAlertPayload**](AddTagsToAlertPayload.md) | Request payload of creating alert tags action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_team

> crate::models::SuccessResponse add_team(identifier, add_team_to_alert_payload, identifier_type)
Add Team

Add team to alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**add_team_to_alert_payload** | [**AddTeamToAlertPayload**](AddTeamToAlertPayload.md) | Request payload of adding team to alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_alert

> crate::models::SuccessResponse assign_alert(identifier, assign_alert_payload, identifier_type)
Assign Alert

Assign alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**assign_alert_payload** | [**AssignAlertPayload**](AssignAlertPayload.md) | Request payload of assigning alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## close_alert

> crate::models::SuccessResponse close_alert(identifier, identifier_type, close_alert_payload)
Close Alert

Closes alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**close_alert_payload** | Option<[**CloseAlertPayload**](CloseAlertPayload.md)> | Request payload of closing alert action |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_alerts

> crate::models::GetCountAlertsResponse count_alerts(query, search_identifier, search_identifier_type)
Count Alerts

Count alerts in Opsgenie

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Search query to apply while filtering the alerts |  |
**search_identifier** | Option<**String**> | Identifier of the saved search query to apply while filtering the alerts |  |
**search_identifier_type** | Option<**String**> | Identifier type of the saved search query. Possible values are id and name. Default value is id. If searchIdentifier is not provided, this value is ignored. |  |[default to id]

### Return type

[**crate::models::GetCountAlertsResponse**](GetCountAlertsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alert

> crate::models::SuccessResponse create_alert(create_alert_payload)
Create Alert

Creates a new alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_alert_payload** | [**CreateAlertPayload**](CreateAlertPayload.md) | Request payload of created alert | [required] |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_saved_searches

> crate::models::CreateSavedSearchResponse create_saved_searches(create_saved_search_payload)
Create Saved Search

Create saved search with given fields

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_saved_search_payload** | [**CreateSavedSearchPayload**](CreateSavedSearchPayload.md) | Request payload of creating saved search | [required] |

### Return type

[**crate::models::CreateSavedSearchResponse**](CreateSavedSearchResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert

> crate::models::SuccessResponse delete_alert(identifier, identifier_type, user, source)
Delete Alert

Deletes an alert using alert id, tiny id or alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**user** | Option<**String**> | Display name of the request owner |  |
**source** | Option<**String**> | Display name of the request source |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saved_search

> crate::models::SuccessResponse delete_saved_search(identifier, identifier_type)
Delete Saved Search

Deletes saved search using given search identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of the saved search which could be 'id' or 'name' | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', or 'name' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## escalate_alert

> crate::models::SuccessResponse escalate_alert(identifier, escalate_alert_to_next_payload, identifier_type)
Escalate Alert

Escalate alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**escalate_alert_to_next_payload** | [**EscalateAlertToNextPayload**](EscalateAlertToNextPayload.md) | Request payload of escalating alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_custom_alert_action

> crate::models::SuccessResponse execute_custom_alert_action(identifier, action_name, identifier_type, execute_custom_alert_action_payload)
Custom Alert Action

Custom actions for the alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**action_name** | **String** | Name of the action to execute | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**execute_custom_alert_action_payload** | Option<[**ExecuteCustomAlertActionPayload**](ExecuteCustomAlertActionPayload.md)> | Request payload of executing custom alert action |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert

> crate::models::GetAlertResponse get_alert(identifier, identifier_type)
Get Alert

Returns alert with given id, tiny id or alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::GetAlertResponse**](GetAlertResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachment

> crate::models::GetAlertAttachmentResponse get_attachment(identifier, attachment_id, alert_identifier_type)
Get Alert Attachment

Get alert attachment name and url for the given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**attachment_id** | **i64** | Identifier of alert attachment | [required] |
**alert_identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::GetAlertAttachmentResponse**](GetAlertAttachmentResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request_status

> crate::models::GetRequestStatusResponse get_request_status(request_id)
Get Request Status of Alert

Used to track the status and alert details (if any) of the request whose identifier is given

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Universally unique identifier of the questioned request | [required] |

### Return type

[**crate::models::GetRequestStatusResponse**](GetRequestStatusResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saved_search

> crate::models::GetSavedSearchResponse get_saved_search(identifier, identifier_type)
Get Saved Search

Get saved search for the given search identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of the saved search which could be 'id' or 'name' | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', or 'name' |  |[default to id]

### Return type

[**crate::models::GetSavedSearchResponse**](GetSavedSearchResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alerts

> crate::models::ListAlertsResponse list_alerts(query, search_identifier, search_identifier_type, offset, limit, sort, order)
List Alerts

Returns list of alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Search query to apply while filtering the alerts |  |
**search_identifier** | Option<**String**> | Identifier of the saved search query to apply while filtering the alerts |  |
**search_identifier_type** | Option<**String**> | Identifier type of the saved search query. Possible values are 'id', or 'name' |  |[default to id]
**offset** | Option<**i32**> | Start index of the result set (to apply pagination). Minimum value (and also default value) is 0 |  |
**limit** | Option<**i32**> | Maximum number of items to provide in the result. Must be a positive integer value. Default value is 20 and maximum value is 100 |  |
**sort** | Option<**String**> | Name of the field that result set will be sorted by |  |[default to createdAt]
**order** | Option<**String**> | Sorting order of the result set |  |[default to desc]

### Return type

[**crate::models::ListAlertsResponse**](ListAlertsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachments

> crate::models::ListAlertAttachmentsResponse list_attachments(identifier, alert_identifier_type)
List Alert Attachments

List alert attachment names and urls for related alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**alert_identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::ListAlertAttachmentsResponse**](ListAlertAttachmentsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_logs

> crate::models::ListAlertLogsResponse list_logs(identifier, identifier_type, offset, direction, limit, order)
List Alert Logs

List alert logs for the given alert identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**offset** | Option<**String**> | Starting value of the offset property |  |
**direction** | Option<**String**> | Page direction to apply for the given offset with 'next' and 'prev' |  |[default to next]
**limit** | Option<**i32**> | Maximum number of items to provide in the result. Must be a positive integer value. Default value is 20 and maximum value is 100 |  |
**order** | Option<**String**> | Sorting order of the result set |  |[default to desc]

### Return type

[**crate::models::ListAlertLogsResponse**](ListAlertLogsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notes

> crate::models::ListAlertNotesResponse list_notes(identifier, identifier_type, offset, direction, limit, order)
List Alert Notes

List alert notes for the given alert identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**offset** | Option<**String**> | Starting value of the offset property |  |
**direction** | Option<**String**> | Page direction to apply for the given offset with 'next' and 'prev' |  |[default to next]
**limit** | Option<**i32**> | Maximum number of items to provide in the result. Must be a positive integer value. Default value is 20 and maximum value is 100 |  |
**order** | Option<**String**> | Sorting order of the result set |  |[default to desc]

### Return type

[**crate::models::ListAlertNotesResponse**](ListAlertNotesResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recipients

> crate::models::ListAlertRecipientsResponse list_recipients(identifier, identifier_type)
List Alert Recipients

List alert recipients for the given alert identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::ListAlertRecipientsResponse**](ListAlertRecipientsResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saved_searches

> crate::models::ListSavedSearchesResponse list_saved_searches()
Lists Saved Searches

List all saved searches

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListSavedSearchesResponse**](ListSavedSearchesResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_attachment

> crate::models::SuccessResponse remove_attachment(identifier, attachment_id, alert_identifier_type, user)
Remove Alert Attachment

Remove alert attachment for the given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**attachment_id** | **i64** | Identifier of alert attachment | [required] |
**alert_identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**user** | Option<**String**> | Display name of the request owner |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_details

> crate::models::SuccessResponse remove_details(identifier, keys, identifier_type, user, note, source)
Remove Details

Remove details of the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**keys** | [**Vec<String>**](String.md) | Comma separated list of keys to remove from the custom properties of the alert (e.g. 'key1,key2') | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**user** | Option<**String**> | Display name of the request owner |  |
**note** | Option<**String**> | Additional alert note to add |  |
**source** | Option<**String**> | Display name of the request source |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_tags

> crate::models::SuccessResponse remove_tags(identifier, tags, identifier_type, user, note, source)
Remove Tags

Remove tags of the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**tags** | [**Vec<String>**](String.md) | Tags field of the given alert as comma seperated values (e.g. 'tag1, tag2') | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**user** | Option<**String**> | Display name of the request owner |  |
**note** | Option<**String**> | Additional alert note to add |  |
**source** | Option<**String**> | Display name of the request source |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snooze_alert

> crate::models::SuccessResponse snooze_alert(identifier, snooze_alert_payload, identifier_type)
Snooze Alert

Snooze alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**snooze_alert_payload** | [**SnoozeAlertPayload**](SnoozeAlertPayload.md) | Request payload of snoozing alert action | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_acknowledge_alert

> crate::models::SuccessResponse un_acknowledge_alert(identifier, identifier_type, un_acknowledge_alert_payload)
UnAcknowledge Alert

UnAcknowledge alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]
**un_acknowledge_alert_payload** | Option<[**UnAcknowledgeAlertPayload**](UnAcknowledgeAlertPayload.md)> | Request payload of unacknowledging alert action |  |

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alert_description

> crate::models::SuccessResponse update_alert_description(identifier, update_alert_description_payload, identifier_type)
Update Alert Description

Update the description of the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**update_alert_description_payload** | [**UpdateAlertDescriptionPayload**](UpdateAlertDescriptionPayload.md) | Request payload of update alert description | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alert_message

> crate::models::SuccessResponse update_alert_message(identifier, update_alert_message_payload, identifier_type)
Update Alert Message

Update the message of the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**update_alert_message_payload** | [**UpdateAlertMessagePayload**](UpdateAlertMessagePayload.md) | Request payload of update alert message | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alert_priority

> crate::models::SuccessResponse update_alert_priority(identifier, update_alert_priority_payload, identifier_type)
Update Alert Priority

Update the priority of the alert with given identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of alert which could be alert id, tiny id or alert alias | [required] |
**update_alert_priority_payload** | [**UpdateAlertPriorityPayload**](UpdateAlertPriorityPayload.md) | Request payload of update alert priority | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', 'alias' or 'tiny' |  |[default to id]

### Return type

[**crate::models::SuccessResponse**](SuccessResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saved_search

> crate::models::GetSavedSearchResponse update_saved_search(identifier, update_saved_search_payload, identifier_type)
Update Saved Search

Update saved search for the given search identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | **String** | Identifier of the saved search which could be 'id' or 'name' | [required] |
**update_saved_search_payload** | [**UpdateSavedSearchPayload**](UpdateSavedSearchPayload.md) | Request payload of updating saved search | [required] |
**identifier_type** | Option<**String**> | Type of the identifier that is provided as an in-line parameter. Possible values are 'id', or 'name' |  |[default to id]

### Return type

[**crate::models::GetSavedSearchResponse**](GetSavedSearchResponse.md)

### Authorization

[GenieKey](../README.md#GenieKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

