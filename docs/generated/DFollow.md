# DFollow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"dfollow\". | 
**instrument_name** | **String** | Name of the instrument to trade, in order to follow the deltas of the target. Must be an outright instrument with at least 0.25 delta.  | 
**target_instrument** | **String** | Name of the outright option to follow the deltas of. | 
**target_amount** | **f64** | Amount of `target_instrument` contracts to follow the deltas of. Must be between 0.1 and 1000.  | 
**threshold** | Option<**f64**> | Delta correction threshold. Defaults to 0. Must be between 0 and 0.3. | [optional]
**tolerance** | Option<**f64**> | Maximum deviation allowed from target deltas at any time. | [optional]
**period** | **f64** | Number of seconds to let the deltas of the following position stay outside of `[target-threshold, target+threshold]`, before correcting them, where `target = deltas of target_instrument * target_amount`. Must be between 1 and 3600.  | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot should stop executing. Must not be further away than a week. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


