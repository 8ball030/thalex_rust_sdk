# OrderHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** | System order ID, use to identify in amend/cancel, and to match trades. | 
**order_type** | **String** |  | 
**instrument_name** | Option<**String**> | Order instrument name.  Not present for combination orders. Refer to `legs` field instead.  | [optional]
**legs** | Option<[**Vec<models::OrderHistoryLegsInner>**](OrderHistory_legs_inner.md)> | List of legs for this combination order.  Not present for single-leg orders. Refer to `instrument_name` field instead.  | [optional]
**direction** | **String** |  | 
**price** | Option<**f64**> | Limit price. May be omitted if no price was supplied (e.g. for a market order).  For combination orders this specifies limit price per unit of the combination.  | [optional]
**amount** | **f64** | Order size (as inserted or amended to).  For combination orders this specifies the amount of units of the combination to trade.  | 
**filled_amount** | **f64** | Part of the order that has been executed.  For combination orders this specifies the amount of units of the combination filled. Legs are filled proportionally to their quantities.  | 
**label** | Option<**String**> | Label supplied with insert. Can also be a number. Field is omitted if no label was supplied. | [optional]
**client_order_id** | Option<**f64**> | Client-supplied order id. Field is omitted if no client order id was supplied. | [optional]
**status** | **String** |  | 
**fills** | [**Vec<models::OrderFill>**](OrderFill.md) | All fills for this order. | 
**delete_reason** | **String** | Detailed reason of order deletion if the order was deleted, omitted otherwise.  The following reasons are possible:  - `client_cancel`: Order was cancelled by the client, e.g. with a call to `private/cancel`.  - `client_bulk_cancel`: Order was cancelled by the client with a bulk cancel call, e.g. `private/cancel_all`.  - `session_end`: Non-persistent order was automatically cancelled when a WebSocket session ended.  - `instrument_deactivated`: Order was automatically cancelled when the order instrument was deactivated,   for example after expiration.  - `mm_protection`: Order was automatically cancelled when configured market maker protection amount was exhausted.  - `failover`: Non-persistent order was automatically cancelled on matching engine failover.  - `margin_breach`: Order was automatically cancelled in a response to a margin breach on the account as part   of automatic liquidation procedures.  - `filled`: Order was filled in full.  - `immediate_cancel`: The order was submitted as \"immediate-or-cancel\" and was not filled in full immediately.   Note that the order might be partially filled when this delete reason is set.  - `admin_cancel`: The order was cancelled by an exchange admin.  | 
**insert_reason** | **String** | Detailed reason of order insertion. | 
**conditional_order_id** | Option<**String**> | If the order was triggered by a conditional order (stop order), the ID of that conditional order. Otherwise omitted.  | [optional]
**bot_id** | Option<**String**> | If the order was inserted by a bot, the ID of that bot. Otherwise omitted.  | [optional]
**create_time** | **f64** | Creation time (Unix timestamp). | 
**close_time** | **f64** | Time when this order was closed or canceled (Unix timestamp). | 
**reduce_only** | Option<**bool**> | True if the order is a reduce only order, omitted otherwise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


