# RestPrivateAmendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_order_id** | Option<**i32**> | Exactly one of `client_order_id` or `order_id` must be specified. | [optional]
**order_id** | Option<**String**> | Exactly one of `client_order_id` or `order_id` must be specified. | [optional]
**price** | **f64** |  | 
**amount** | **f64** |  | 
**collar** | Option<**String**> | If the instrument has a safety price collar set, and the new limit price is in cross with (more aggressive than) this collar, how to handle. If set to `ignore`, the amend will proceed as requested. If `reject`, the request fails early. If `clamp`, the price is adjusted to the collar.  The default is `reject`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


