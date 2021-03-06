# BaseAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**tiny_id** | Option<**String**> |  | [optional]
**alias** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**acknowledged** | Option<**bool**> |  | [optional]
**is_seen** | Option<**bool**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**snoozed** | Option<**bool**> |  | [optional]
**snoozed_until** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**count** | Option<**i32**> |  | [optional]
**last_occurred_at** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**created_at** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**updated_at** | Option<[**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md)> |  | [optional]
**source** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**priority** | Option<**String**> |  | [optional]
**responders** | Option<[**Vec<crate::models::Responder>**](Responder.md)> |  | [optional]
**integration** | Option<[**crate::models::AlertIntegration**](AlertIntegration.md)> |  | [optional]
**report** | Option<[**crate::models::AlertReport**](AlertReport.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


