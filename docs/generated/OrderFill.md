# OrderFill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | **String** | Trade ID. | 
**price** | **f64** | Fill price. | 
**amount** | **f64** | Filled amount | 
**time** | Option<**f64**> |  | [optional]
**maker_taker** | **String** | Maker (trade on book order) or taker (trade on new order), if applicable. | 
**leg_index** | **f64** | Index of a leg on which the trade happened for combination orders. Zero for single-leg orders.  For combination orders the direction of the trade is defined by the direction of the order and the sign of the leg quantity.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


