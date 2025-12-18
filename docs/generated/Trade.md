# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_type** | Option<**String**> | Type of the trade.  Note: as of API v2.31.0 we have stopped representing futures-style settlements as trades of `daily_mark` type. You might still get such trades in the history, but no new trades of `daily_mark` type will be created. To get information about daily marks, use `private/daily_mark_history` API endpoint.  | [optional]
**trade_id** | Option<**String**> |  | [optional]
**order_id** | Option<**String**> |  | [optional]
**instrument_name** | Option<**String**> |  | [optional]
**direction** | Option<**String**> |  | [optional]
**price** | Option<**f64**> | Trade price. | [optional]
**amount** | Option<**f64**> | Traded amount. | [optional]
**label** | Option<**String**> | User label. | [optional]
**time** | Option<**f64**> | Time of trade (Unix timestamp). | [optional]
**position_after** | Option<**f64**> | Position in this instrument right after the trade. | [optional]
**session_realised_after** | Option<**f64**> | Session realised P&L for this instrument right after the trade. | [optional]
**position_pnl** | Option<**f64**> | If trade closed a position, the positional P&L that was realised. | [optional]
**perpetual_funding_pnl** | Option<**f64**> | If trade closed a position in a perpetual, the funding P&L that was realised. | [optional]
**fee** | Option<**f64**> | The fee paid for this trade. | [optional]
**index** | Option<**f64**> | The relevant index at time of trade. | [optional]
**fee_rate** | Option<**f64**> | The fee rate applied to calculate the fee. | [optional]
**funding_mark** | Option<**f64**> | The perpetual funding mark as applied to the trade (see `Ticker`). | [optional]
**liquidation_fee** | Option<**f64**> | Fee paid in case of liquidation. | [optional]
**client_order_id** | Option<**f64**> | Client order reference as set in related order. | [optional]
**maker_taker** | Option<**String**> | Maker (trade on book order) or taker (trade on new order), if applicable. | [optional]
**bot_id** | Option<**String**> | If the trade was made by a bot, the ID of that bot. Otherwise omitted.  | [optional]
**leg_index** | Option<**f64**> | Index of a leg on which the trade happened for combination orders. Zero for single-leg orders.  For combination orders the direction of the trade is defined by the direction of the order and the sign of the leg quantity.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


