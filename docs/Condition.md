# Condition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Specifies which alert field will be used in condition. Possible values are message, alias, description, source, entity, tags, actions, extra-properties, recipients or teams | 
**key** | Option<**String**> | If field is set as extra-properties, key could be used for key-value pair | [optional]
**not** | Option<**bool**> | Indicates behaviour of the given operation. Default value is false | [optional]
**operation** | **String** | It is the operation that will be executed for the given field and key. | 
**expected_value** | Option<**String**> | User defined value that will be compared with alert field according to the operation. Default value is empty string | [optional]
**order** | Option<**i64**> | Order of the condition in conditions list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


