# \ConditionalOrdersApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_cancel_all_conditional_orders**](ConditionalOrdersApi.md#rest_private_slash_cancel_all_conditional_orders) | **POST** /private/cancel_all_conditional_orders | Bulk cancel conditional orders
[**rest_private_slash_cancel_conditional_order**](ConditionalOrdersApi.md#rest_private_slash_cancel_conditional_order) | **POST** /private/cancel_conditional_order | Cancel conditional order
[**rest_private_slash_conditional_orders**](ConditionalOrdersApi.md#rest_private_slash_conditional_orders) | **GET** /private/conditional_orders | Conditional orders
[**rest_private_slash_create_conditional_order**](ConditionalOrdersApi.md#rest_private_slash_create_conditional_order) | **POST** /private/create_conditional_order | Create conditional order



## rest_private_slash_cancel_all_conditional_orders

> models::ErrorResponse rest_private_slash_cancel_all_conditional_orders(body)
Bulk cancel conditional orders

Exchange: `https://thalex.com/api/v2/private/cancel_all_conditional_orders`  Testnet: `https://testnet.thalex.com/api/v2/private/cancel_all_conditional_orders`  Cancel all conditional orders of this subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_cancel_conditional_order

> models::ErrorResponse rest_private_slash_cancel_conditional_order(rest_private_cancel_conditional_order_request)
Cancel conditional order

Exchange: `https://thalex.com/api/v2/private/cancel_conditional_order`  Testnet: `https://testnet.thalex.com/api/v2/private/cancel_conditional_order`  Cancel conditional order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_cancel_conditional_order_request** | Option<[**RestPrivateCancelConditionalOrderRequest**](RestPrivateCancelConditionalOrderRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_conditional_orders

> models::RestPrivateConditionalOrders200Response rest_private_slash_conditional_orders()
Conditional orders

Exchange: `https://thalex.com/api/v2/private/conditional_orders`  Testnet: `https://testnet.thalex.com/api/v2/private/conditional_orders`  Get conditional orders

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateConditionalOrders200Response**](rest_private_conditional_orders_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_create_conditional_order

> models::RestPrivateCreateConditionalOrder200Response rest_private_slash_create_conditional_order(rest_private_create_conditional_order_request)
Create conditional order

Exchange: `https://thalex.com/api/v2/private/create_conditional_order`  Testnet: `https://testnet.thalex.com/api/v2/private/create_conditional_order`  A buy order will activate when a trade/mark/index happens at a price at or higher than the stop price, or at or lower than the bracket price (if set). A sell order will activate when a trigger happens at a price at or lower than the stop price, or at or higher than the bracket price (if set).  When a callback rate is set, the stop price for a buy order will trail down at (trade price * (1 + callback rate)), and for a sell order the stop price will trail up at (trade price * (1 - callback rate)).  A stop limit order will activate to a good-till-cancelled limit order, all other types will activate to a market order.  The `last` trigger target tracks aggressive trades in the instrument. The `mark` target tracks the mark price of the instrument, as calculated every second by the Thalex pricing engine. The `index` trigger target is allowed only for futures (instrument type `perpetual` or `future`), and tracks the index price of the respective underlying, as calculated every second by the Thalex pricing engine.  Only the following combinations are possible:  * stop order: set only stop price * stop limit order: set stop price and limit price * bracket order: set stop price and bracket price. For a buy order, the bracket price must be under the stop price, and   for a sell order the other way around. * trailing stop loss order: set stop price and callback rate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_create_conditional_order_request** | Option<[**RestPrivateCreateConditionalOrderRequest**](RestPrivateCreateConditionalOrderRequest.md)> |  |  |

### Return type

[**models::RestPrivateCreateConditionalOrder200Response**](rest_private_create_conditional_order_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

