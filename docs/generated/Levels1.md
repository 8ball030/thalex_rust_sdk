# Levels1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | **String** | Individually identifies this bot instance. You can use it to cancel this specific one. | 
**status** | **String** |  | 
**stop_reason** | Option<**String**> | The reason why the bot stopped executing. Possible values are [\"client_cancel\", \"client_bulk_cancel\", \"end_time\", \"instrument_deactivated\", \"margin_breach\", \"admin_cancel\", \"conflict\", \"strategy\"]. | [optional]
**strategy** | **String** | equal to \"levels\" | 
**instrument_name** | **String** | Name of the instrument the bot instance is trading. | 
**bids** | **Vec<f64>** | The default price levels the bot will be bidding at. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**asks** | **Vec<f64>** | The default price levels the bot will be quoting on the ask side. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**step_size** | **f64** | The default size of to quote on one level. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**base_position** | **f64** | Will be used as a reference to compare the subaccount's portfolio position in the instrument that this bot is trading to. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**upside_exit_price** | **f64** | If the mark price of the instrument this bot is trading goes above `upside_exit_price`, the bot cancels the maker orders, aggressively trades into `base_position`, and then stops executing. | 
**downside_exit_price** | **f64** | If the mark price of the instrument this bot is trading goes below `downside_exit_price`, the bot cancels the maker orders, aggressively trades into `base_position`, and then stops executing. | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade when exiting any position, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot stops executing, cancelling its orders and leaving all positions of the subaccount intact. | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


