# \HistoricalDataApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_public_slash_index_price_historical_data**](HistoricalDataApi.md#rest_public_slash_index_price_historical_data) | **GET** /public/index_price_historical_data | Index price historical data.
[**rest_public_slash_mark_price_historical_data**](HistoricalDataApi.md#rest_public_slash_mark_price_historical_data) | **GET** /public/mark_price_historical_data | Mark price historical data.



## rest_public_slash_index_price_historical_data

> models::RestPublicIndexPriceHistoricalData200Response rest_public_slash_index_price_historical_data(index_name, from, to, resolution)
Index price historical data.

Exchange: `https://thalex.com/api/v2/public/index_price_historical_data`  Testnet: `https://testnet.thalex.com/api/v2/public/index_price_historical_data`  Returns index price historical data in the specified interval and resolution in OHLC format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** |  | [required] |
**from** | **f64** |  | [required] |
**to** | **f64** |  | [required] |
**resolution** | **String** |  | [required] |

### Return type

[**models::RestPublicIndexPriceHistoricalData200Response**](rest_public_index_price_historical_data_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_mark_price_historical_data

> models::RestPublicMarkPriceHistoricalData200Response rest_public_slash_mark_price_historical_data(instrument_name, from, to, resolution)
Mark price historical data.

Exchange: `https://thalex.com/api/v2/public/mark_price_historical_data`  Testnet: `https://testnet.thalex.com/api/v2/public/mark_price_historical_data`  Returns mark price historical data in the specified interval and resolution in OHLC format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument_name** | **String** |  | [required] |
**from** | **f64** |  | [required] |
**to** | **f64** |  | [required] |
**resolution** | **String** |  | [required] |

### Return type

[**models::RestPublicMarkPriceHistoricalData200Response**](rest_public_mark_price_historical_data_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

