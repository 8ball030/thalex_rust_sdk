# PortfolioEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument_name** | Option<**String**> | Instrument name. | [optional]
**position** | Option<**f64**> | Amount of this contract currently held; short positions are negative. | [optional]
**mark_price** | Option<**f64**> | Current mark price for the instrument. | [optional]
**iv** | Option<**f64**> | Implied volatility calculated at time of marking. | [optional]
**index** | Option<**f64**> | Index price at time of marking. | [optional]
**start_price** | Option<**f64**> | Average price paid to obtain position.  Note: for instruments that are subject to daily futures-style settlement, the start price is reset to the mark price at the end of each session and all the unrealized P&L is thus realized. Use `private/daily_mark_history` API endpoint to get information about daily settlements.  | [optional]
**average_price** | Option<**f64**> | Average price paid to obtain position. Doesn't reset at settlement. | [optional]
**unrealised_pnl** | Option<**f64**> | Unrealised P&L in the current session for this position based on current mark price, equal to `(mark_price - start_price) * position`.  Note: for instruments that are subject to daily futures-style settlement, the start price is reset to the mark price at the end of each session and all the unrealized P&L is thus realized. Use `private/daily_mark_history` API endpoint to get information about daily settlements.  | [optional]
**realised_pnl** | Option<**f64**> | Realized P&L in the current session.  Realized P&L is settled into a settlement asset at the end of each session.   | [optional]
**entry_value** | Option<**f64**> | Total entry value, equal to `start_price * position`. | [optional]
**perpetual_funding_entry_value** | Option<**f64**> | Entry mark value for perpetual funding. Unrealised perpetual funding is (current perp funding mark * position) - perpetual funding entry value. Not included if zero.  | [optional]
**unrealised_perpetual_funding** | Option<**f64**> | For perpetual positions, current unrealized perpetual funding.  The funding is realized as P&L and settled into settlement asset at the end of each session.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


