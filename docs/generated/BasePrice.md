# BasePrice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_name** | **String** | Instrument name of future, or \"synthetic\". | 
**price** | **f64** | Forward price of the expiration. | 
**index** | **f64** | Index price of the underlying of the instrument. | 
**index_delta** | Option<**f64**> | Before expiration, Thalex linearly scales the deltas of the expiring instruments to zero, over a time window. This number represents how far into this process the expiration is (i.e. delta used for margining = index_delta * actual delta of the instrument).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


