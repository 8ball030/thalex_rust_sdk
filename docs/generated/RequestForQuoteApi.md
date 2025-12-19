# \RequestForQuoteApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_cancel_rfq**](RequestForQuoteApi.md#rest_private_slash_cancel_rfq) | **POST** /private/cancel_rfq | Cancel an RFQ
[**rest_private_slash_create_rfq**](RequestForQuoteApi.md#rest_private_slash_create_rfq) | **POST** /private/create_rfq | Create a request for quote
[**rest_private_slash_open_rfqs**](RequestForQuoteApi.md#rest_private_slash_open_rfqs) | **GET** /private/open_rfqs | Open RFQs
[**rest_private_slash_trade_rfq**](RequestForQuoteApi.md#rest_private_slash_trade_rfq) | **POST** /private/trade_rfq | Trade an RFQ



## rest_private_slash_cancel_rfq

> models::ErrorResponse rest_private_slash_cancel_rfq(rest_private_cancel_rfq_request)
Cancel an RFQ

Exchange: `https://thalex.com/api/v2/private/cancel_rfq`  Testnet: `https://testnet.thalex.com/api/v2/private/cancel_rfq`  Cancels the indicated RFQ.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_cancel_rfq_request** | Option<[**RestPrivateCancelRfqRequest**](RestPrivateCancelRfqRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_create_rfq

> models::RestPrivateCreateRfq200Response rest_private_slash_create_rfq(rest_private_create_rfq_request)
Create a request for quote

Exchange: `https://thalex.com/api/v2/private/create_rfq`  Testnet: `https://testnet.thalex.com/api/v2/private/create_rfq`  Creates a new RFQ. You do not have to indicate upfront whether you want to buy or sell this package. Indicate the full size of the package. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_create_rfq_request** | Option<[**RestPrivateCreateRfqRequest**](RestPrivateCreateRfqRequest.md)> |  |  |

### Return type

[**models::RestPrivateCreateRfq200Response**](rest_private_create_rfq_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_open_rfqs

> models::RestPrivateOpenRfqs200Response rest_private_slash_open_rfqs()
Open RFQs

Exchange: `https://thalex.com/api/v2/private/open_rfqs`  Testnet: `https://testnet.thalex.com/api/v2/private/open_rfqs`  Retrieves a list of open RFQs created by this account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateOpenRfqs200Response**](rest_private_open_rfqs_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_trade_rfq

> models::RestPrivateTradeRfq200Response rest_private_slash_trade_rfq(rest_private_trade_rfq_request)
Trade an RFQ

Exchange: `https://thalex.com/api/v2/private/trade_rfq`  Testnet: `https://testnet.thalex.com/api/v2/private/trade_rfq`  Trade on the quotes given. The requested amount is that of the original request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_trade_rfq_request** | Option<[**RestPrivateTradeRfqRequest**](RestPrivateTradeRfqRequest.md)> |  |  |

### Return type

[**models::RestPrivateTradeRfq200Response**](rest_private_trade_rfq_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

