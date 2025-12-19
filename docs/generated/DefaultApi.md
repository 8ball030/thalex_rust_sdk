# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ticker_instrument_delay_get**](DefaultApi.md#ticker_instrument_delay_get) | **GET** /ticker/{instrument}/{delay} | Subscribe to ticker data for a given instrument or combination



## ticker_instrument_delay_get

> models::TickerResponse ticker_instrument_delay_get(instrument, delay)
Subscribe to ticker data for a given instrument or combination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | **String** |  | [required] |
**delay** | [**Delay**](.md) | Minimum interval between feeds | [required] |

### Return type

[**models::TickerResponse**](TickerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

