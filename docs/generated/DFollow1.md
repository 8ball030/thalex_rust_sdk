# DFollow1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"dfollow\". | 
**instrument_name** | **String** | Name of the instrument to trade, in order to follow the deltas of the target. | 
**target_instrument** | **String** | Name of the option to follow the deltas of. | 
**target_amount** | **f64** | Amount of `target_instrument` contracts to follow the deltas of. | 
**threshold** | **f64** | Tolerance threshold. | 
**period** | **f64** | Number of seconds to let the deltas of the following position stay outside of `[target-threshold, target+threshold]`, before correcting them, where `target = deltas of target_instrument * target_amount`.  | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot should stop executing. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


