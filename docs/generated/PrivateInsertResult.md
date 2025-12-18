# PrivateInsertResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** |  | 
**order_type** | **String** |  | 
**time_in_force** | **String** |  | 
**instrument_name** | Option<**String**> |  | [optional]
**legs** | Option<[**Vec<models::PrivateInsertInlinePropertiesLegsItems>**](PrivateInsertInlinePropertiesLegsItems.md)> |  | [optional]
**direction** | **String** |  | 
**price** | Option<**f64**> |  | [optional]
**amount** | **f64** |  | 
**filled_amount** | **f64** |  | 
**remaining_amount** | **f64** |  | 
**label** | Option<**String**> |  | [optional]
**client_order_id** | Option<**f64**> |  | [optional]
**status** | **String** |  | 
**fills** | [**Vec<models::OrderFill>**](OrderFill.md) |  | 
**change_reason** | **String** |  | 
**delete_reason** | Option<**String**> |  | [optional]
**insert_reason** | **String** |  | 
**conditional_order_id** | Option<**String**> |  | [optional]
**bot_id** | Option<**String**> |  | [optional]
**create_time** | **f64** |  | 
**close_time** | Option<**f64**> |  | [optional]
**reduce_only** | Option<**bool**> |  | [optional]
**persistent** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


