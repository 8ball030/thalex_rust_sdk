# OrderStatusLegsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument_name** | **String** | Leg instrument name.  | 
**quantity** | **f64** | Quantity of this leg in a unit of the combination. A non-zero integer, negative for short legs.  | 
**filled_amount** | **f64** | Amount executed on this leg.  Legs are filled proportionally to their quantities.  | 
**remaining_amount** | **f64** | Amount on this leg that remains in the book; if 0, order is now inactive.  Note that the only `time_in_force` for combination orders currently supported is `immediate_or_cancel`. Therefore, combination orders will never remain on a book, and this field will always be zero.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


