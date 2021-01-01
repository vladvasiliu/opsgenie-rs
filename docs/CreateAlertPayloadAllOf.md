# CreateAlertPayloadAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | Message of the alert | 
**alias** | Option<**String**> | Client-defined identifier of the alert, that is also the key element of alert deduplication. | [optional]
**description** | Option<**String**> | Description field of the alert that is generally used to provide a detailed information about the alert. | [optional]
**responders** | Option<[**Vec<crate::models::Recipient>**](Recipient.md)> | Responders that the alert will be routed to send notifications | [optional]
**visible_to** | Option<[**Vec<crate::models::Recipient>**](Recipient.md)> | Teams and users that the alert will become visible to without sending any notification | [optional]
**actions** | Option<**Vec<String>**> | Custom actions that will be available for the alert | [optional]
**tags** | Option<**Vec<String>**> | Tags of the alert | [optional]
**details** | Option<**::std::collections::HashMap<String, String>**> | Map of key-value pairs to use as custom properties of the alert | [optional]
**entity** | Option<**String**> | Entity field of the alert that is generally used to specify which domain alert is related to | [optional]
**priority** | Option<**String**> | Priority level of the alert | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


