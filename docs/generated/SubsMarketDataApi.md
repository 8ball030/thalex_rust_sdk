# \SubsMarketDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subscribe_account_conditional_orders**](SubsMarketDataApi.md#subscribe_account_conditional_orders) | **GET** /account/conditional_orders | Subscribe to account.conditional_orders channel
[**subscribe_account_order_history**](SubsMarketDataApi.md#subscribe_account_order_history) | **GET** /account/order_history | Subscribe to account.order_history channel
[**subscribe_account_orders**](SubsMarketDataApi.md#subscribe_account_orders) | **GET** /account/orders | Subscribe to account.orders channel
[**subscribe_account_persistent_orders**](SubsMarketDataApi.md#subscribe_account_persistent_orders) | **GET** /account/persistent_orders | Subscribe to account.persistent_orders channel
[**subscribe_account_portfolio**](SubsMarketDataApi.md#subscribe_account_portfolio) | **GET** /account/portfolio | Subscribe to account.portfolio channel
[**subscribe_account_rfq_history**](SubsMarketDataApi.md#subscribe_account_rfq_history) | **GET** /account/rfq_history | Subscribe to account.rfq_history channel
[**subscribe_account_rfqs**](SubsMarketDataApi.md#subscribe_account_rfqs) | **GET** /account/rfqs | Subscribe to account.rfqs channel
[**subscribe_account_summary**](SubsMarketDataApi.md#subscribe_account_summary) | **GET** /account/summary | Subscribe to account.summary channel
[**subscribe_account_trade_history**](SubsMarketDataApi.md#subscribe_account_trade_history) | **GET** /account/trade_history | Subscribe to account.trade_history channel
[**subscribe_base_price_less_than_underlying_greater_than_less_than_expiration_greater_than**](SubsMarketDataApi.md#subscribe_base_price_less_than_underlying_greater_than_less_than_expiration_greater_than) | **GET** /base_price/{underlying}/{expiration} | Subscribe to base_price.<underlying>.<expiration> channel
[**subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_book_less_than_instrument_greater_than_less_than_grouping_greater_than_less_than_nlevels_greater_than_less_than_delay_greater_than) | **GET** /book/{instrument}/{grouping}/{nlevels}/{delay} | Subscribe to book.<instrument>.<grouping>.<nlevels>.<delay> channel
[**subscribe_index_components_less_than_underlying_greater_than**](SubsMarketDataApi.md#subscribe_index_components_less_than_underlying_greater_than) | **GET** /index_components/{underlying} | Subscribe to index_components.<underlying> channel
[**subscribe_instruments**](SubsMarketDataApi.md#subscribe_instruments) | **GET** /instruments | Subscribe to instruments channel
[**subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_lwt_less_than_instrument_greater_than_less_than_delay_greater_than) | **GET** /lwt/{instrument}/{delay} | Subscribe to lwt.<instrument>.<delay> channel
[**subscribe_mm_rfq_quotes**](SubsMarketDataApi.md#subscribe_mm_rfq_quotes) | **GET** /mm/rfq_quotes | Subscribe to mm.rfq_quotes channel
[**subscribe_mm_rfqs**](SubsMarketDataApi.md#subscribe_mm_rfqs) | **GET** /mm/rfqs | Subscribe to mm.rfqs channel
[**subscribe_price_index_less_than_underlying_greater_than**](SubsMarketDataApi.md#subscribe_price_index_less_than_underlying_greater_than) | **GET** /price_index/{underlying} | Subscribe to price_index.<underlying> channel
[**subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than**](SubsMarketDataApi.md#subscribe_recent_trades_less_than_target_greater_than_less_than_category_greater_than) | **GET** /recent_trades/{target}/{category} | Subscribe to recent_trades.<target>.<category> channel
[**subscribe_rfqs**](SubsMarketDataApi.md#subscribe_rfqs) | **GET** /rfqs | Subscribe to rfqs channel
[**subscribe_session_mm_protection**](SubsMarketDataApi.md#subscribe_session_mm_protection) | **GET** /session/mm_protection | Subscribe to session.mm_protection channel
[**subscribe_session_orders**](SubsMarketDataApi.md#subscribe_session_orders) | **GET** /session/orders | Subscribe to session.orders channel
[**subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than**](SubsMarketDataApi.md#subscribe_ticker_less_than_instrument_greater_than_less_than_delay_greater_than) | **GET** /ticker/{instrument}/{delay} | Subscribe to ticker.<instrument>.<delay> channel
[**subscribe_underlying_statistics_less_than_underlying_greater_than**](SubsMarketDataApi.md#subscribe_underlying_statistics_less_than_underlying_greater_than) | **GET** /underlying_statistics/{underlying} | Subscribe to underlying_statistics.<underlying> channel



## subscribe_account_conditional_orders

> models::AccountNotification subscribe_account_conditional_orders()
Subscribe to account.conditional_orders channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_order_history

> models::AccountNotification subscribe_account_order_history()
Subscribe to account.order_history channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_orders

> models::AccountNotification subscribe_account_orders()
Subscribe to account.orders channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_persistent_orders

> models::AccountNotification subscribe_account_persistent_orders()
Subscribe to account.persistent_orders channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_portfolio

> models::AccountNotification subscribe_account_portfolio()
Subscribe to account.portfolio channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_rfq_history

> models::AccountNotification subscribe_account_rfq_history()
Subscribe to account.rfq_history channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_rfqs

> models::AccountNotification subscribe_account_rfqs()
Subscribe to account.rfqs channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_summary

> models::AccountNotification subscribe_account_summary()
Subscribe to account.summary channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_account_trade_history

> models::AccountNotification subscribe_account_trade_history()
Subscribe to account.trade_history channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_base_price_less_than_underlying_greater_than_less_than_expiration_greater_than

> models::BasePriceNotification subscribe_base_price_less_than_underlying_greater_than_less_than_expiration_greater_than(underlying, expiration)
Subscribe to base_price.<underlying>.<expiration> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**underlying** | **String** |  | [required] |
**expiration** | **String** |  | [required] |

### Return type

[**models::BasePriceNotification**](BasePriceNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## subscribe_index_components_less_than_underlying_greater_than

> models::IndexComponentsNotification subscribe_index_components_less_than_underlying_greater_than(underlying)
Subscribe to index_components.<underlying> channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**underlying** | **String** |  | [required] |

### Return type

[**models::IndexComponentsNotification**](IndexComponentsNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_instruments

> models::InstrumentsPayloadNotification subscribe_instruments()
Subscribe to instruments channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstrumentsPayloadNotification**](InstrumentsPayloadNotification.md)

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


## subscribe_mm_rfq_quotes

> models::MmNotification subscribe_mm_rfq_quotes()
Subscribe to mm.rfq_quotes channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MmNotification**](MmNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_mm_rfqs

> models::MmNotification subscribe_mm_rfqs()
Subscribe to mm.rfqs channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MmNotification**](MmNotification.md)

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


## subscribe_rfqs

> models::RfqsPayloadNotification subscribe_rfqs()
Subscribe to rfqs channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RfqsPayloadNotification**](RfqsPayloadNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_session_mm_protection

> models::SessionNotification subscribe_session_mm_protection()
Subscribe to session.mm_protection channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SessionNotification**](SessionNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_session_orders

> models::SessionNotification subscribe_session_orders()
Subscribe to session.orders channel

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SessionNotification**](SessionNotification.md)

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

