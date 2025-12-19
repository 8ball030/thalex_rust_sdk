# \SubsMarketDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than) | **GET** /book/{instrument}/{grouping}/{nlevels}/{delay} | Subscribe to book.<instrument>.<grouping>.<nlevels>.<delay> channel
[**subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than) | **GET** /lwt/{instrument}/{delay} | Subscribe to lwt.<instrument>.<delay> channel
[**subscribe_price_index_less_than_underlying_greater_than**](SubsMarketDataApi.md#subscribe_price_index_less_than_underlying_greater_than) | **GET** /price_index/{underlying} | Subscribe to price_index.<underlying> channel
[**subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than**](SubsMarketDataApi.md#subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than) | **GET** /recent_trades/{target}/{category} | Subscribe to recent_trades.<target>.<category> channel
[**subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than) | **GET** /ticker/{instrument}/{delay} | Subscribe to ticker.<instrument>.<delay> channel
[**subscribe_underlying_statistics_less_than_underlying_greater_than**](SubsMarketDataApi.md#subscribe_underlying_statistics_less_than_underlying_greater_than) | **GET** /underlying_statistics/{underlying} | Subscribe to underlying_statistics.<underlying> channel



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


## subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than

> models::LwtNotification subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than(instrument, delay)
Subscribe to lwt.<instrument>.<delay> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | **String** |  | [required] |
**delay** | [**Delay**](.md) |  | [required] |

### Return type

[**models::LwtNotification**](LwtNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_price_index_less_than_underlying_greater_than

> models::PriceIndexNotification subscribe_price_index_less_than_underlying_greater_than(underlying)
Subscribe to price_index.<underlying> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**underlying** | **String** |  | [required] |

### Return type

[**models::PriceIndexNotification**](PriceIndexNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than

> models::RecentTradesNotification subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than(target, category)
Subscribe to recent_trades.<target>.<category> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | **String** |  | [required] |
**category** | **String** |  | [required] |

### Return type

[**models::RecentTradesNotification**](RecentTradesNotification.md)

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


## subscribe_underlying_statistics_less_than_underlying_greater_than

> models::UnderlyingStatisticsNotification subscribe_underlying_statistics_less_than_underlying_greater_than(underlying)
Subscribe to underlying_statistics.<underlying> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**underlying** | **String** |  | [required] |

### Return type

[**models::UnderlyingStatisticsNotification**](UnderlyingStatisticsNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

