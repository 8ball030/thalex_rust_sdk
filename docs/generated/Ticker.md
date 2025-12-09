# Ticker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**best_bid_price** | Option<**f64**> | Price of the best (highest) bid in the orderbook, or `null` if the orderbook is empty. | [optional]
**best_bid_amount** | Option<**f64**> | Size of best bid, or `null` if the orderbook is empty. | [optional]
**best_ask_price** | Option<**f64**> | Price of best (lowest) ask in the orderbook, or `null` if the orderbook is empty. | [optional]
**best_ask_amount** | Option<**f64**> | Size of best ask, or `null` if the orderbook is empty. | [optional]
**last_price** | Option<**f64**> | Price of last trade, or `null` if no trades have been registered yet.  Not included for combinations.  | [optional]
**mark_price** | Option<**f64**> | Current mark price. | [optional]
**mark_timestamp** | Option<**f64**> | The unix timestamp when the price was marked. | [optional]
**iv** | Option<**f64**> | Implied volatility calculated at time of marking.  Only included for options. Not included for combinations.  | [optional]
**delta** | Option<**f64**> | Delta calculated at time of marking.  Not included for combinations.  | [optional]
**index** | Option<**f64**> | Index price at time of marking. | [optional]
**forward** | Option<**f64**> | Forward price at time of marking.  Only included for options.  | [optional]
**volume_24h** | Option<**f64**> | Total volume traded over the last 24 hours.  Not included for combinations.  | [optional]
**value_24h** | Option<**f64**> | Total value traded over the last 24 hours.  Not included for combinations.  | [optional]
**low_price_24h** | Option<**f64**> | Lowest price in the last 24 hours.  Not included for combinations.  | [optional]
**high_price_24h** | Option<**f64**> | Highest price in the last 24 hours.  Not included for combinations.  | [optional]
**change_24h** | Option<**f64**> | Difference in price between the first and the last trades in the last 24 hours, `null` if there were no trades.  Not included for combinations.  | [optional]
**collar_low** | Option<**f64**> | Current price collar low (checks new asks) | [optional]
**collar_high** | Option<**f64**> | Current price collar high (checks new bids) | [optional]
**open_interest** | Option<**f64**> | Total number of outstanding unsettled contracts.  Not included for combinations.  | [optional]
**funding_rate** | Option<**f64**> | Current rate at which long position pays and short position earns, in funding interval.  Only included for perpetuals.  | [optional]
**funding_mark** | Option<**f64**> | Funding value of a single contract long position since last settlement.  Only included for perpetuals.  | [optional]
**realised_funding_24h** | Option<**f64**> | Total funding accumulated for a single contract long position over the last 24 hours.  Only included for perpetuals.  | [optional]
**average_funding_rate_24h** | Option<**f64**> | Average funding rate for the last 24 hours.  Only included for perpetuals.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


