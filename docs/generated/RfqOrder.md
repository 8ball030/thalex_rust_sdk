# RfqOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | Identifier of the RFQ. | 
**order_id** | **String** | Identifier of the individual quote. | 
**client_order_id** | Option<**f64**> | Client-supplied order id. Field is omitted if no client order id was supplied. | [optional]
**direction** | **String** |  | 
**price** | **f64** |  | 
**amount** | **f64** |  | 
**label** | Option<**String**> |  | [optional]
**trade_price** | Option<**f64**> | the price at which this order traded. | [optional]
**trade_amount** | Option<**f64**> | the number of combinations that traded. | [optional]
**delete_reason** | Option<**String**> | Detailed reason of order deletion. | [optional]
**event** | Option<**String**> | This field is set only on subscriptions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


