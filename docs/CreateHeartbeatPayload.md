# CreateHeartbeatPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the heartbeat | 
**description** | Option<**String**> | An optional description of the heartbeat | [optional]
**interval** | **i32** | Specifies how often a heartbeat message should be expected | 
**interval_unit** | **String** | Interval specified as 'minutes', 'hours' or 'days' | 
**enabled** | **bool** | Enable/disable heartbeat monitoring | 
**owner_team** | Option<[**crate::models::CreateHeartbeatPayloadAllOfOwnerTeam**](CreateHeartbeatPayload_allOf_ownerTeam.md)> |  | [optional]
**alert_message** | Option<**String**> | Specifies the alert message for heartbeat expiration alert. If this is not provided, default alert message is 'HeartbeatName is expired' | [optional]
**alert_tags** | Option<**Vec<String>**> | Specifies the alert tags for heartbeat expiration alert | [optional]
**alert_priority** | Option<**String**> | Specifies the alert priority for heartbeat expiration alert. If this is not provided, default priority is P3 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


