# Sgsl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"sgsl\" | 
**instrument_name** | **String** | Name of the instrument to run SGSL on. | 
**signal** | **String** |  | 
**entry_price** | **f64** | Price to compare `signal` price to, to determine necessary adjustments to the portfolio. See [the bot strategies section](#tag/bot_strategies) for more explanation. Must be greater or equal to `exit_price + 5 * price tick of the instrument`.  | 
**target_position** | **f64** | The target position to maintain in the subaccount if `signal` price is above `entry_price`. See [the bot strategies section](#tag/bot_strategies) for more explanation. Can be maximum `500 * volume tick of the instrument` away from `exit_position`.  | 
**exit_price** | **f64** | Price to compare `signal` price to, to determine necessary adjustments to the portfolio. See [the bot strategies section](#tag/bot_strategies) for more explanation. Must be lower or equal to `entry_price - 5 * price tick of the instrument`.  | 
**exit_position** | **f64** | The target position to maintain in the subaccount if `signal` price is below `exit_price`. See [the bot strategies section](#tag/bot_strategies) for more explanation. Can be maximum `500 * volume tick of the instrument` away from `target_position`.  | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot should stop executing. Must not be further away than a week. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


