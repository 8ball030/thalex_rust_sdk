# ConditionalOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** | Unique ID, use to identify in cancel | 
**instrument_name** | **String** |  | 
**direction** | **String** |  | 
**amount** | **f64** | Size of the order when activated | 
**target** | **String** | The trigger type that `stop_price` and `bracket_price` will refer to. | 
**stop_price** | **f64** | Trigger price at which the order will be activated | 
**limit_price** | Option<**f64**> | For stop limit order, the price at which the order will be placed | [optional]
**bracket_price** | Option<**f64**> | For bracket order, the price at which profit will be taken (upper activation price) | [optional]
**trailing_stop_callback_rate** | Option<**f64**> | For trailing stop loss, the callback rate as a ratio (e.g. 0.05 for 5%) | [optional]
**label** | Option<**String**> | Optional user label | [optional]
**status** | **String** |  | 
**create_time** | **f64** |  | 
**update_time** | **f64** | Time of last update (conversion or change of trailing stop price) | 
**convert_time** | Option<**f64**> | Time of trigger | [optional]
**converted_order_id** | Option<**String**> | System order ID of the created order. | [optional]
**reject_reason** | **String** | If conversion failed, the reason | 
**reduce_only** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


