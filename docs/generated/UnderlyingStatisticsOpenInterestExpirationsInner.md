# UnderlyingStatisticsOpenInterestExpirationsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expiration_timestamp** | Option<**i32**> | Expiration time as Unix timestamp (seconds). | [optional]
**expiry_date** | Option<**String**> | Expiration date in ISO format (YYYY-mm-dd). | [optional]
**call** | Option<**f64**> | Total count of outstanding unsettled call options for the given expiration and underlying. | [optional]
**put** | Option<**f64**> | Total count of outstanding unsettled put options for the given expiration and underlying. | [optional]
**option** | Option<**f64**> | Total count of outstanding unsettled options for the given expiration and underlying. | [optional]
**future** | Option<**f64**> | Total count of outstanding unsettled future contracts for the given expiration and underlying. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


