# SnoozeAlertPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**String**> | Display name of the request owner | [optional]
**note** | Option<**String**> | Additional note that will be added while creating the alert | [optional]
**source** | Option<**String**> | Source field of the alert. Default value is IP address of the incoming request | [optional]
**end_time** | [**chrono::DateTime<chrono::offset::Utc>**](chrono::DateTime<chrono::offset::Utc>.md) | Date and time that snooze will lose effect | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


