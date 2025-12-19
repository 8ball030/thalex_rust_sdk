# \SubsMarketDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than) | **GET** /book/{instrument}/{grouping}/{nlevels}/{delay} | Subscribe to book.<instrument>.<grouping>.<nlevels>.<delay> channel
[**subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than) | **GET** /ticker/{instrument}/{delay} | Subscribe to ticker.<instrument>.<delay> channel



## subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than

> models::BookNotification subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than(instrument, grouping, nlevels, delay)
Subscribe to book.<instrument>.<grouping>.<nlevels>.<delay> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | **String** |  | [required] |
**grouping** | **String** |  | [required] |
**nlevels** | **String** |  | [required] |
**delay** | [**Delay**](.md) |  | [required] |

### Return type

[**models::BookNotification**](BookNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than

> models::TickerNotification subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than(instrument, delay)
Subscribe to ticker.<instrument>.<delay> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | **String** |  | [required] |
**delay** | [**Delay**](.md) |  | [required] |

### Return type

[**models::TickerNotification**](TickerNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

