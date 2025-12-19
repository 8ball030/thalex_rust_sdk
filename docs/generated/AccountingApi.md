# \AccountingApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_account_breakdown**](AccountingApi.md#rest_private_slash_account_breakdown) | **GET** /private/account_breakdown | Account breakdown
[**rest_private_slash_account_summary**](AccountingApi.md#rest_private_slash_account_summary) | **GET** /private/account_summary | Account summary
[**rest_private_slash_daily_mark_history**](AccountingApi.md#rest_private_slash_daily_mark_history) | **GET** /private/daily_mark_history | Daily mark history
[**rest_private_slash_open_orders**](AccountingApi.md#rest_private_slash_open_orders) | **GET** /private/open_orders | Open orders
[**rest_private_slash_order_history**](AccountingApi.md#rest_private_slash_order_history) | **GET** /private/order_history | Order history
[**rest_private_slash_portfolio**](AccountingApi.md#rest_private_slash_portfolio) | **GET** /private/portfolio | Portfolio
[**rest_private_slash_required_margin_breakdown**](AccountingApi.md#rest_private_slash_required_margin_breakdown) | **GET** /private/required_margin_breakdown | Margin breakdown
[**rest_private_slash_required_margin_for_order**](AccountingApi.md#rest_private_slash_required_margin_for_order) | **GET** /private/required_margin_for_order | Margin breakdown with order
[**rest_private_slash_rfq_history**](AccountingApi.md#rest_private_slash_rfq_history) | **GET** /private/rfq_history | RFQ history
[**rest_private_slash_trade_history**](AccountingApi.md#rest_private_slash_trade_history) | **GET** /private/trade_history | Trade history
[**rest_private_slash_transaction_history**](AccountingApi.md#rest_private_slash_transaction_history) | **GET** /private/transaction_history | Transaction history



## rest_private_slash_account_breakdown

> models::RestPrivateAccountBreakdown200Response rest_private_slash_account_breakdown()
Account breakdown

Exchange: `https://thalex.com/api/v2/private/account_breakdown`  Testnet: `https://testnet.thalex.com/api/v2/private/account_breakdown`  Get account breakdown

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateAccountBreakdown200Response**](rest_private_account_breakdown_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_account_summary

> models::RestPrivateAccountSummary200Response rest_private_slash_account_summary()
Account summary

Exchange: `https://thalex.com/api/v2/private/account_summary`  Testnet: `https://testnet.thalex.com/api/v2/private/account_summary`  Get account summary

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateAccountSummary200Response**](rest_private_account_summary_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_daily_mark_history

> models::RestPrivateDailyMarkHistory200Response rest_private_slash_daily_mark_history(limit, time_low, time_high, bookmark)
Daily mark history

Exchange: `https://thalex.com/api/v2/private/daily_mark_history`  Testnet: `https://testnet.thalex.com/api/v2/private/daily_mark_history`  For instruments that are subject to futures-style settlement we perform daily settlement at the mark price. The settlement procedure realizes the positional and perpetual funding profits/losses accumulated during the session, and resets the start price of the position to the mark price.  This API endpoint returns a historical log of settled profits/losses (daily marks). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |
**bookmark** | Option<**String**> |  |  |

### Return type

[**models::RestPrivateDailyMarkHistory200Response**](rest_private_daily_mark_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_open_orders

> models::RestPrivateOpenOrders200Response rest_private_slash_open_orders()
Open orders

Exchange: `https://thalex.com/api/v2/private/open_orders`  Testnet: `https://testnet.thalex.com/api/v2/private/open_orders`  Get open orders

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateOpenOrders200Response**](rest_private_open_orders_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_order_history

> models::RestPrivateOrderHistory200Response rest_private_slash_order_history(limit, time_low, time_high, bookmark, sort, instrument_names, bot_ids)
Order history

Exchange: `https://thalex.com/api/v2/private/order_history`  Testnet: `https://testnet.thalex.com/api/v2/private/order_history`  Retrieves a list of past orders (i.e. orders that are not active anymore) since the last 90 days. Allows sorting and filtering by instrument name.  Unfilled market maker orders are not included.  Orders are like order status updates, without the `remaining_amount` field (always 0), and with a `close_time` timestamp. Note that, for technical reasons, the 'fills' field in the order is limited to a length of 8. For a full list of trades, refer to trade history.  Note that it is not real-time, data might appear with a slight delay. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |
**bookmark** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**instrument_names** | Option<**String**> |  |  |
**bot_ids** | Option<**String**> |  |  |

### Return type

[**models::RestPrivateOrderHistory200Response**](rest_private_order_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_portfolio

> models::RestPrivatePortfolio200Response rest_private_slash_portfolio()
Portfolio

Exchange: `https://thalex.com/api/v2/private/portfolio`  Testnet: `https://testnet.thalex.com/api/v2/private/portfolio`  Get account portfolio

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivatePortfolio200Response**](rest_private_portfolio_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_required_margin_breakdown

> models::RestPrivateRequiredMarginBreakdown200Response rest_private_slash_required_margin_breakdown()
Margin breakdown

Exchange: `https://thalex.com/api/v2/private/required_margin_breakdown`  Testnet: `https://testnet.thalex.com/api/v2/private/required_margin_breakdown`  Get margin breakdown

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateRequiredMarginBreakdown200Response**](rest_private_required_margin_breakdown_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_required_margin_for_order

> models::RestPrivateRequiredMarginForOrder200Response rest_private_slash_required_margin_for_order(price, amount, instrument_name, legs)
Margin breakdown with order

Exchange: `https://thalex.com/api/v2/private/required_margin_for_order`  Testnet: `https://testnet.thalex.com/api/v2/private/required_margin_for_order`  This method returns a lightweight breakdown of the account as it is, and also as if a hypothetical order of a given price and amount would be inserted on either side of the book.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price** | **f64** |  | [required] |
**amount** | **f64** |  | [required] |
**instrument_name** | Option<**String**> |  |  |
**legs** | Option<[**Vec<models::RestPrivateRequiredMarginForOrderLegsParameterInner>**](models::RestPrivateRequiredMarginForOrderLegsParameterInner.md)> |  |  |

### Return type

[**models::RestPrivateRequiredMarginForOrder200Response**](rest_private_required_margin_for_order_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_rfq_history

> models::RestPrivateRfqHistory200Response rest_private_slash_rfq_history(limit, time_low, time_high, bookmark)
RFQ history

Exchange: `https://thalex.com/api/v2/private/rfq_history`  Testnet: `https://testnet.thalex.com/api/v2/private/rfq_history`  Retrieves a list of past RFQs for the account. Open RFQs are not incuded. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |
**bookmark** | Option<**String**> |  |  |

### Return type

[**models::RestPrivateRfqHistory200Response**](rest_private_rfq_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_trade_history

> models::RestPrivateTradeHistory200Response rest_private_slash_trade_history(limit, time_low, time_high, bookmark, sort, instrument_names, bot_ids)
Trade history

Exchange: `https://thalex.com/api/v2/private/trade_history`  Testnet: `https://testnet.thalex.com/api/v2/private/trade_history`  Retrieves trades for the last 90 days. Allows sorting and filtering by instrument name. Note that it is not real-time, trades might appear with a slight delay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |
**bookmark** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**instrument_names** | Option<**String**> |  |  |
**bot_ids** | Option<**String**> |  |  |

### Return type

[**models::RestPrivateTradeHistory200Response**](rest_private_trade_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_transaction_history

> models::RestPrivateTransactionHistory200Response rest_private_slash_transaction_history(limit, time_low, time_high, bookmark)
Transaction history

Exchange: `https://thalex.com/api/v2/private/transaction_history`  Testnet: `https://testnet.thalex.com/api/v2/private/transaction_history`  Get transaction history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]
**time_low** | Option<**f64**> |  |  |
**time_high** | Option<**f64**> |  |  |
**bookmark** | Option<**String**> |  |  |

### Return type

[**models::RestPrivateTransactionHistory200Response**](rest_private_transaction_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

