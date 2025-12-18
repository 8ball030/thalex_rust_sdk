# RestPrivateCreateConditionalOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | **String** |  | 
**instrument_name** | **String** |  | 
**amount** | **f64** |  | 
**limit_price** | Option<**f64**> | If set, creates a stop limit order | [optional]
**target** | Option<**String**> | The trigger target that `stop_price` and `bracket_price` refer to. | [optional]
**stop_price** | **f64** | Trigger price | 
**bracket_price** | Option<**f64**> | If set, creates a bracket order | [optional]
**trailing_stop_callback_rate** | Option<**f64**> | If set, creates a trailing stop order | [optional]
**label** | Option<**String**> | Label will be set on the activated order | [optional]
**reduce_only** | Option<**bool**> | Activated order will be reduce-only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


