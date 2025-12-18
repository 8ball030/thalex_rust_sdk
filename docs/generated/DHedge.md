# DHedge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"dhedge\". | 
**instrument_name** | **String** | Name of the instrument the bot will trade. Must be an outright instrument with at least 0.25 delta.  | 
**position** | Option<**String**> | Name of an outright instrument. If specified, the bot will only hedge the position in this instrument, not the entire underlying. | [optional]
**target_delta** | Option<**f64**> | Delta to target. Defaults to 0. Must be between -10 and 10. | [optional]
**threshold** | Option<**f64**> | Hedging threshold. Defaults to 0. Must be between 0 and 0.3. | [optional]
**tolerance** | Option<**f64**> | Maximum deviation allowed from target deltas at any time. | [optional]
**period** | **f64** | Number of seconds to let the deltas stay outside of `[target-threshold, target+threshold]`, before hedging them. Must be between 1 and 3600.  | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade, expressed as % of the traded instruments mark price. | [optional]
**end_time** | Option<**f64**> | Timestamp when the bot should stop executing. If not specified, the bot will run until it's manually stopped. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | [optional]
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


