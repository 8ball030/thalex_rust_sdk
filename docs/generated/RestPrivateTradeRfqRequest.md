# RestPrivateTradeRfqRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | The ID of the RFQ | 
**direction** | **String** | Whether to buy or sell. *Important*: this relates to the combination as created by the system, *not* the package as originally requested (although they should be equal).  | 
**limit_price** | **f64** | The maximum (for buy) or minimum (for sell) price to trade at. This is the price for one combination, not for the entire package.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


