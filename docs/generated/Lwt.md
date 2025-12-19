# Lwt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**b** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Best bid (if any). | [optional]
**a** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Best ask (if any). | [optional]
**m** | Option<**f64**> | Mark price. | [optional]
**v** | Option<**f64**> | Mark volatility.  Only included for options. Not included for combinations.  | [optional]
**l** | Option<**f64**> | Price of last trade (if any).  Not included for combinations.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


