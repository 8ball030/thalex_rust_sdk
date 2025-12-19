# UnderlyingStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_volume_24h** | Option<**f64**> | Total amount traded in 24 hours, including futures and options. | [optional]
**option_volume_24h** | Option<**f64**> | Total amount of options traded in the past 24 hours. | [optional]
**delta_one_volume_24h** | Option<**f64**> | Total amount of futures traded in the past 24 hours. | [optional]
**trade_value_24h** | Option<**f64**> | Total underlying value traded in 24 hours. This is the sum of (amount * index price at time of trade). | [optional]
**option_value_24h** | Option<**f64**> | Total option value traded in 24 hours. This is the sum of (amount * index price at time of trade). | [optional]
**delta_one_value_24h** | Option<**f64**> | Total delta one value traded in 24 hours. This is the sum of (amount * index price at time of trade). | [optional]
**open_interest** | Option<[**models::UnderlyingStatisticsOpenInterest**](UnderlyingStatistics_open_interest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


