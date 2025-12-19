# Index

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index_name** | **String** |  | 
**price** | **f64** |  | 
**timestamp** | **f64** | The unix timestamp when the index price was recorded in the database. | 
**expiration_print_average** | Option<**f64**> | The average price so far over the current expiration, if any. | [optional]
**expiration_progress** | Option<**f64**> | A number between 0.0 and 1.0 indicating the progress of the current expiration. | [optional]
**expected_expiration_price** | Option<**f64**> | If expiration is in progress, and the index price will not change any more, this is the expiration price. Equals `expiration_progress * expiration_print_average + (1 - expiration_progress) * price`.  | [optional]
**previous_settlement_price** | Option<**f64**> | The last known settlement price (expiration price, underlying delivery price). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


