# Incident

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**tiny_id** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**is_seen** | Option<**bool**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**created_at** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**updated_at** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**source** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**priority** | Option<**String**> |  | [optional]
**responders** | Option<[**Vec<crate::models::Responder>**](Responder.md)> |  | [optional]
**owner_team** | Option<**String**> |  | [optional]
**extra_properties** | Option<**::std::collections::HashMap<String, String>**> | Map of key-value pairs to use as custom properties of the incident | [optional]
**request_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


