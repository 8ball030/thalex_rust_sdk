# Grid

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"grid\". | 
**instrument_name** | **String** | Name of the instrument to run the Grid bot on. | 
**grid** | **Vec<f64>** | The default price levels the bot will be quoting at. Must have minimum 2, maximum 8 levels. Distance between quote levels must minimum 5, maximum 100 ticks. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**step_size** | **f64** | The default size to quote on one level. Must not be greater than 100 volume ticks. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**base_position** | Option<**f64**> | Will be used as a reference to compare the subaccount's portfolio position in the instrument that this bot is trading to. Defaults to the portfolio position of the subaccount in `instrument_name` at the time of sending the request. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | [optional]
**upside_exit_price** | **f64** | If the mark price of the instrument this bot is trading goes above `upside_exit_price`, the bot cancels the maker orders, aggressively trades into `base_position`, and then stops executing. Must not be more than 200 ticks away from highest quote.  | 
**downside_exit_price** | **f64** | If the mark price of the instrument this bot is trading goes below `downside_exit_price`, the bot cancels the maker orders, aggressively trades into `base_position`, and then stops executing. Must not be more than 200 ticks away from lowest quote.  | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade when exiting any position, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot should stop executing. Must not be further away than a week. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


