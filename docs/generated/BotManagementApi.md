# \BotManagementApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_bots**](BotManagementApi.md#rest_private_slash_bots) | **GET** /private/bots | Get bots
[**rest_private_slash_create_bot**](BotManagementApi.md#rest_private_slash_create_bot) | **POST** /private/create_bot | Create a bot



## rest_private_slash_bots

> models::RestPrivateBots200Response rest_private_slash_bots()
Get bots

Exchange: `https://thalex.com/api/v2/private/bots`  Testnet: `https://testnet.thalex.com/api/v2/private/bots`  Get bots

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateBots200Response**](rest_private_bots_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_create_bot

> models::RestPrivateCreateBot200Response rest_private_slash_create_bot(rest_private_create_bot_request)
Create a bot

Exchange: `https://thalex.com/api/v2/private/create_bot`  Testnet: `https://testnet.thalex.com/api/v2/private/create_bot`  Instantiate a bot that keeps continually trading in your name according to a predefined strategy. See [the bot strategies section](#tag/bot_strategies) for more info on bots. For risk fencing reasons and because of the complex ways manual trades can interact with bot strategies, you might want to consider running bots on a separate/dedicated sub account. Also be aware that you can set up bots with different strategies in a way that they would end up trading with each other. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_create_bot_request** | Option<[**RestPrivateCreateBotRequest**](RestPrivateCreateBotRequest.md)> |  |  |

### Return type

[**models::RestPrivateCreateBot200Response**](rest_private_create_bot_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

