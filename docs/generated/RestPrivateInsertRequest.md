# RestPrivateInsertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_order_id** | Option<**i32**> | Session-local identifier for this order. Only valid for websocket sessions. If set, must be an integer between 0 and 2^64-1, inclusive. When using numbers larger than 2^32, please beware of implicit floating point conversions in some JSON libraries.  | [optional]
**instrument_name** | Option<**String**> | Name of the instrument to trade.  This field must not be present when inserting a combination order. Use `legs` field instead.  | [optional]
**legs** | Option<[**Vec<models::InsertRequestLegsInner>**](InsertRequest_legs_inner.md)> | List of legs for a combination order.  There must be at least two and at most four legs specified. All leg instruments must be distinct.  Other constraints apply, please check trading information page on combination orders.  This field must not be present when inserting single-leg orders. Use `instrument_name` field instead.  | [optional]
**price** | Option<**f64**> | Limit price; required for limit orders.  For combination orders this specifies limit price per unit of the combination.  | [optional]
**amount** | **f64** | Amount of currency to trade (e.g. BTC for futures).  For combination orders this specifies the amount of units of the combination to trade.  | 
**label** | Option<**String**> |  | [optional]
**order_type** | Option<**String**> |  | [optional][default to Limit]
**time_in_force** | Option<**String**> | Note that for limit orders, the default `time_in_force` is `good_till_cancelled`, while for market orders, the default is `immediate_or_cancel`. It is illegal to send a GTC market order, or an IOC post order.  For combination orders `time_in_force` must always be set to `immediate_or_cancel`.  | [optional]
**post_only** | Option<**bool**> | If the order price is in cross with the current best price on the opposite side in the order book, then the price is adjusted to one tick away from that price, ensuring that the order will never trade on insert. If the adjusted price of a buy order falls at or below zero where not allowed, then the order is cancelled with delete reason 'immediate_cancel'.  This flag is not supported for combination orders.  | [optional]
**reject_post_only** | Option<**bool**> | This flag is only effective in combination with post_only. If set, then instead of adjusting the order price, the order will be cancelled with delete reason 'immediate_cancel'. The combination of post_only and reject_post_only is effectively a book-or-cancel order.  This flag is not supported for combination orders.  | [optional]
**reduce_only** | Option<**bool**> | An order marked `reduce_only` will have its amount reduced to the open position. If there is no open position, or if the order direction would cause an increase of the open position, the order is rejected. If the order is placed in the book, it will be subsequently monitored, and reduced to the open position if the position changes through other means (best effort). Multiple reduce-only orders will all be reduced individually.  This flag is not supported for combination orders.  | [optional]
**collar** | Option<**String**> | If the instrument has a safety price collar set, and the limit price of the order (infinite for market orders) is in cross with (more aggressive than) this collar, how to handle. If set to `ignore`, the order will proceed as requested. If `reject`, the order fails early. If `clamp`, the price is adjusted to the collar.  The default is `clamp` for market orders and `reject` for everything else.  Collar `ignore` is forbidden for market orders.  Price collars are applied to combination orders. Price collar for a combination is a linear combination of the leg collars with their corresponding quantities as coefficients.  | [optional]
**direction** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


