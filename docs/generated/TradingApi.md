# \TradingApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_amend**](TradingApi.md#rest_private_slash_amend) | **POST** /private/amend | Amend order
[**rest_private_slash_buy**](TradingApi.md#rest_private_slash_buy) | **POST** /private/buy | Insert buy order
[**rest_private_slash_cancel**](TradingApi.md#rest_private_slash_cancel) | **POST** /private/cancel | Cancel order
[**rest_private_slash_cancel_all**](TradingApi.md#rest_private_slash_cancel_all) | **POST** /private/cancel_all | Bulk cancel all orders
[**rest_private_slash_insert**](TradingApi.md#rest_private_slash_insert) | **POST** /private/insert | Insert order
[**rest_private_slash_sell**](TradingApi.md#rest_private_slash_sell) | **POST** /private/sell | Insert sell order



## rest_private_slash_amend

> models::RestPrivateInsert200Response rest_private_slash_amend(rest_private_amend_request)
Amend order

Exchange: `https://thalex.com/api/v2/private/amend`  Testnet: `https://testnet.thalex.com/api/v2/private/amend`  Note that `amount` designates the new \"original\" amount, i.e. the amend is volume-safe. If the specified amount is lower than the already executed amount, the order is deleted.  If the price of the order is the same as the previous price, and the amount is less than the previous amount, book priority is preserved.  If the `amount` is amended to a value at or below the executed amount, the order is cancelled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_amend_request** | Option<[**RestPrivateAmendRequest**](RestPrivateAmendRequest.md)> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_buy

> models::RestPrivateInsert200Response rest_private_slash_buy(insert_request)
Insert buy order

Exchange: `https://thalex.com/api/v2/private/buy`  Testnet: `https://testnet.thalex.com/api/v2/private/buy`  Insert buy order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insert_request** | Option<[**InsertRequest**](InsertRequest.md)> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_cancel

> models::RestPrivateInsert200Response rest_private_slash_cancel(rest_private_cancel_request)
Cancel order

Exchange: `https://thalex.com/api/v2/private/cancel`  Testnet: `https://testnet.thalex.com/api/v2/private/cancel`  Cancel order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_cancel_request** | Option<[**RestPrivateCancelRequest**](RestPrivateCancelRequest.md)> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_cancel_all

> models::RestPrivateInsert200Response rest_private_slash_cancel_all(body)
Bulk cancel all orders

Exchange: `https://thalex.com/api/v2/private/cancel_all`  Testnet: `https://testnet.thalex.com/api/v2/private/cancel_all`  Cancels all orders for the account. This may *not* match new orders in flight (see `private/cancel_session`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_insert

> models::RestPrivateInsert200Response rest_private_slash_insert(rest_private_insert_request)
Insert order

Exchange: `https://thalex.com/api/v2/private/insert`  Testnet: `https://testnet.thalex.com/api/v2/private/insert`  Insert an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_insert_request** | Option<[**RestPrivateInsertRequest**](RestPrivateInsertRequest.md)> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_sell

> models::RestPrivateInsert200Response rest_private_slash_sell(insert_request)
Insert sell order

Exchange: `https://thalex.com/api/v2/private/sell`  Testnet: `https://testnet.thalex.com/api/v2/private/sell`  Insert sell order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insert_request** | Option<[**InsertRequest**](InsertRequest.md)> |  |  |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

