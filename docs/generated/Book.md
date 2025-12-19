# Book

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trades** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List of `[price, amount, direction, timestamp, implied_taker]`. Note that the snapshot of this feed may contain older trades that happened since the last restart of the gateway.  The `implied_taker` is a boolean flag, set to `true` when the actual taker trade happened on another order book, and the maker trade on this book is the result of implied matching.  Trades are not sent for combination order books.  | [optional]
**bid_changes** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List of price level updates (price, amount, outright amount) for buy orders, amount 0 means level is now empty.  For combination order books, price and amount refer to the price and amount per unit of the combination.  | [optional]
**ask_changes** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | List of price level updates (price, amount, outright amount) for sell orders, amount 0 means level is now empty.  For combination order books, price and amount refer to the price and amount per unit of the combination.  | [optional]
**total_bid_amount** | Option<**f64**> | The total amount of bid orders across all levels. | [optional]
**total_ask_amount** | Option<**f64**> | The total amount of ask orders across all levels. | [optional]
**time** | Option<**f64**> | Time of the last recorded update to this order book (Unix timestamp). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


