# RestPrivateMmRfqAmendQuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_order_id** | Option<**i32**> | Exactly one of `client_order_id` or `order_id` must be specified. | [optional]
**order_id** | Option<**String**> | Exactly one of `client_order_id` or `order_id` must be specified. | [optional]
**price** | **f64** | Limit price for the quote (for one combination). | 
**amount** | **f64** | Number of combinations to quote. Anything over the requested amount will not be visible to the requester.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


