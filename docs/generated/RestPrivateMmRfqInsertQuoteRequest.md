# RestPrivateMmRfqInsertQuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | The ID of the RFQ this quote is for. | 
**client_order_id** | Option<**i32**> | Session-local identifier for this order. Only valid for websocket sessions. If set, must be a number between 0 and 2^64-1, inclusive. When using numbers larger than 2^32, please beware of implicit floating point conversions in some JSON libraries.  | [optional]
**direction** | **String** | The side of the quote.  | 
**price** | **f64** | Limit price for the quote (for one combination). | 
**amount** | **f64** | Number of combinations to quote. Anything over the requested amount will not be visible to the requester.  | 
**label** | Option<**String**> | A label to attach to eventual trades. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


