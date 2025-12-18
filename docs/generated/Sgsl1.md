# Sgsl1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | **String** | Individually identifies this bot instance. You can use it to cancel this specific one. | 
**status** | **String** |  | 
**stop_reason** | Option<**String**> | The reason why the bot stopped executing. Possible values are [\"client_cancel\", \"client_bulk_cancel\", \"end_time\", \"instrument_deactivated\", \"margin_breach\", \"admin_cancel\", \"conflict\", \"strategy\"]. | [optional]
**strategy** | **String** | equal to \"sgsl\" | 
**instrument_name** | **String** | Name of the instrument. | 
**signal** | **String** |  | 
**entry_price** | **f64** | Price to compare `signal` price to, to determine necessary adjustments to the portfolio. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**target_position** | **f64** | The target position to maintain in the subaccount if `signal` price is above `entry_price`. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**exit_price** | **f64** | Price to compare `signal` price to, to determine necessary adjustments to the portfolio. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**exit_position** | **f64** | The target position to maintain in the subaccount if `signal` price is below `exit_price`. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**max_slippage** | Option<**f64**> | Maximum slippage per trade, expressed as % of the traded instruments mark price. | [optional]
**end_time** | **f64** | Timestamp when the bot stops executing, cancelling its orders and leaving all positions of the subaccount intact. | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


