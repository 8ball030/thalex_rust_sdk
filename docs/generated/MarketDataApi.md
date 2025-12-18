# \MarketDataApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_public_slash_all_instruments**](MarketDataApi.md#rest_public_slash_all_instruments) | **GET** /public/all_instruments | All instruments
[**rest_public_slash_book**](MarketDataApi.md#rest_public_slash_book) | **GET** /public/book | Single order book
[**rest_public_slash_index**](MarketDataApi.md#rest_public_slash_index) | **GET** /public/index | Single index value
[**rest_public_slash_instrument**](MarketDataApi.md#rest_public_slash_instrument) | **GET** /public/instrument | Single instrument
[**rest_public_slash_instruments**](MarketDataApi.md#rest_public_slash_instruments) | **GET** /public/instruments | Active instruments
[**rest_public_slash_ticker**](MarketDataApi.md#rest_public_slash_ticker) | **GET** /public/ticker | Single ticker value



## rest_public_slash_all_instruments

> models::RestPrivateNotificationsInbox200Response rest_public_slash_all_instruments(time_low, time_high)
All instruments

Exchange: `https://thalex.com/api/v2/public/all_instruments`  Testnet: `https://testnet.thalex.com/api/v2/public/all_instruments`  Retrieves the list of all instruments that were active in the specified time interval.  Note that the time interval cannot be larger than 3 days.  You can also use `public/instrument` call to retrieve information about a specific instrument. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |

### Return type

[**models::RestPrivateNotificationsInbox200Response**](rest_private_notifications_inbox_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_book

> models::RestPrivateCancelAll200Response rest_public_slash_book(instrument_name)
Single order book

Exchange: `https://thalex.com/api/v2/public/book`  Testnet: `https://testnet.thalex.com/api/v2/public/book`  Retrieves aggregated price depth for a single instrument, with a maximum of 5 levels. Please do *not* use this to poll for data -- a websocket subscription is more flexible and more useful. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument_name** | **String** |  | [required] |

### Return type

[**models::RestPrivateCancelAll200Response**](rest_private_cancel_all_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_index

> models::RestPublicIndex200Response rest_public_slash_index(underlying)
Single index value

Exchange: `https://thalex.com/api/v2/public/index`  Testnet: `https://testnet.thalex.com/api/v2/public/index`  Retrieves the index price for a single underlying. If needed repeatedly, please use the `price_index.<underlying>` websocket subscription. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**underlying** | **String** |  | [required] |

### Return type

[**models::RestPublicIndex200Response**](rest_public_index_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_instrument

> models::RestPublicInstrument200Response rest_public_slash_instrument(instrument_name)
Single instrument

Exchange: `https://thalex.com/api/v2/public/instrument`  Testnet: `https://testnet.thalex.com/api/v2/public/instrument`  Retrieves a singe instrument.  Unlike `public/all_instruments`, this API endpoint allows retrieving information about instruments that have expired long time ago. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument_name** | **String** |  | [required] |

### Return type

[**models::RestPublicInstrument200Response**](rest_public_instrument_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_instruments

> models::RestPublicInstruments200Response rest_public_slash_instruments()
Active instruments

Exchange: `https://thalex.com/api/v2/public/instruments`  Testnet: `https://testnet.thalex.com/api/v2/public/instruments`  Retrieves the list of currently active instruments. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPublicInstruments200Response**](rest_public_instruments_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_public_slash_ticker

> models::RestPublicTicker200Response rest_public_slash_ticker(instrument_name)
Single ticker value

Exchange: `https://thalex.com/api/v2/public/ticker`  Testnet: `https://testnet.thalex.com/api/v2/public/ticker`  Retrieves a single ticker for a single instrument. Please do *not* use this to repeatedly poll for data -- a websocket subscription is much more useful. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument_name** | **String** |  | [required] |

### Return type

[**models::RestPublicTicker200Response**](rest_public_ticker_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

