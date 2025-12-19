# \RequestForQuoteMmApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_mm_rfq_amend_quote**](RequestForQuoteMmApi.md#rest_private_slash_mm_rfq_amend_quote) | **POST** /private/mm_rfq_amend_quote | Amend quote
[**rest_private_slash_mm_rfq_delete_quote**](RequestForQuoteMmApi.md#rest_private_slash_mm_rfq_delete_quote) | **POST** /private/mm_rfq_delete_quote | Delete quote
[**rest_private_slash_mm_rfq_insert_quote**](RequestForQuoteMmApi.md#rest_private_slash_mm_rfq_insert_quote) | **POST** /private/mm_rfq_insert_quote | Quote on an RFQ
[**rest_private_slash_mm_rfq_quotes**](RequestForQuoteMmApi.md#rest_private_slash_mm_rfq_quotes) | **GET** /private/mm_rfq_quotes | List of active quotes
[**rest_private_slash_mm_rfqs**](RequestForQuoteMmApi.md#rest_private_slash_mm_rfqs) | **GET** /private/mm_rfqs | Open RFQs



## rest_private_slash_mm_rfq_amend_quote

> models::RestPrivateMmRfqInsertQuote200Response rest_private_slash_mm_rfq_amend_quote(rest_private_mm_rfq_amend_quote_request)
Amend quote

Exchange: `https://thalex.com/api/v2/private/mm_rfq_amend_quote`  Testnet: `https://testnet.thalex.com/api/v2/private/mm_rfq_amend_quote`  Change the amount and price of an existing quote. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_mm_rfq_amend_quote_request** | Option<[**RestPrivateMmRfqAmendQuoteRequest**](RestPrivateMmRfqAmendQuoteRequest.md)> |  |  |

### Return type

[**models::RestPrivateMmRfqInsertQuote200Response**](rest_private_mm_rfq_insert_quote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_mm_rfq_delete_quote

> models::ErrorResponse rest_private_slash_mm_rfq_delete_quote(rest_private_mm_rfq_delete_quote_request)
Delete quote

Exchange: `https://thalex.com/api/v2/private/mm_rfq_delete_quote`  Testnet: `https://testnet.thalex.com/api/v2/private/mm_rfq_delete_quote`  Deletes the indicated RFQ quote. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_mm_rfq_delete_quote_request** | Option<[**RestPrivateMmRfqDeleteQuoteRequest**](RestPrivateMmRfqDeleteQuoteRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_mm_rfq_insert_quote

> models::RestPrivateMmRfqInsertQuote200Response rest_private_slash_mm_rfq_insert_quote(rest_private_mm_rfq_insert_quote_request)
Quote on an RFQ

Exchange: `https://thalex.com/api/v2/private/mm_rfq_insert_quote`  Testnet: `https://testnet.thalex.com/api/v2/private/mm_rfq_insert_quote`  Sends a new quote on the indicated RFQ. This does *not* remove any previous quote: any number of quotes may be active on either side. Note that if the session was set to non-persistent (cancel-on-disconnect), then this quote will also be non-persistent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_mm_rfq_insert_quote_request** | Option<[**RestPrivateMmRfqInsertQuoteRequest**](RestPrivateMmRfqInsertQuoteRequest.md)> |  |  |

### Return type

[**models::RestPrivateMmRfqInsertQuote200Response**](rest_private_mm_rfq_insert_quote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_mm_rfq_quotes

> models::RestPrivateMmRfqQuotes200Response rest_private_slash_mm_rfq_quotes()
List of active quotes

Exchange: `https://thalex.com/api/v2/private/mm_rfq_quotes`  Testnet: `https://testnet.thalex.com/api/v2/private/mm_rfq_quotes`  Retrieves a list of open RFQ quotes across all RFQs.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateMmRfqQuotes200Response**](rest_private_mm_rfq_quotes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_mm_rfqs

> models::RestPrivateOpenRfqs200Response rest_private_slash_mm_rfqs()
Open RFQs

Exchange: `https://thalex.com/api/v2/private/mm_rfqs`  Testnet: `https://testnet.thalex.com/api/v2/private/mm_rfqs`  Retrieves a list of open RFQs that this account has access to.

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

