# \WalletApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_btc_deposit_address**](WalletApi.md#rest_private_slash_btc_deposit_address) | **GET** /private/btc_deposit_address | Bitcoin deposit address
[**rest_private_slash_crypto_deposits**](WalletApi.md#rest_private_slash_crypto_deposits) | **GET** /private/crypto_deposits | Deposits
[**rest_private_slash_crypto_withdrawals**](WalletApi.md#rest_private_slash_crypto_withdrawals) | **GET** /private/crypto_withdrawals | Withdrawals
[**rest_private_slash_eth_deposit_address**](WalletApi.md#rest_private_slash_eth_deposit_address) | **GET** /private/eth_deposit_address | Ethereum deposit address
[**rest_private_slash_internal_transfer**](WalletApi.md#rest_private_slash_internal_transfer) | **POST** /private/internal_transfer | Internal transfer
[**rest_private_slash_verify_internal_transfer**](WalletApi.md#rest_private_slash_verify_internal_transfer) | **POST** /private/verify_internal_transfer | Verify internal transfer
[**rest_private_slash_verify_withdrawal**](WalletApi.md#rest_private_slash_verify_withdrawal) | **GET** /private/verify_withdrawal | Verify if withdrawal is possible
[**rest_private_slash_withdraw**](WalletApi.md#rest_private_slash_withdraw) | **POST** /private/withdraw | Withdraw assets



## rest_private_slash_btc_deposit_address

> models::RestPrivateCryptoDeposits200Response rest_private_slash_btc_deposit_address()
Bitcoin deposit address

Exchange: `https://thalex.com/api/v2/private/btc_deposit_address`  Testnet: `https://testnet.thalex.com/api/v2/private/btc_deposit_address`  Get Bitcoin deposit address

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateCryptoDeposits200Response**](rest_private_crypto_deposits_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_crypto_deposits

> models::RestPrivateCryptoDeposits200Response rest_private_slash_crypto_deposits()
Deposits

Exchange: `https://thalex.com/api/v2/private/crypto_deposits`  Testnet: `https://testnet.thalex.com/api/v2/private/crypto_deposits`  Pending and confirmed deposits for the selected account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateCryptoDeposits200Response**](rest_private_crypto_deposits_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_crypto_withdrawals

> models::RestPrivateTradeHistory200Response rest_private_slash_crypto_withdrawals()
Withdrawals

Exchange: `https://thalex.com/api/v2/private/crypto_withdrawals`  Testnet: `https://testnet.thalex.com/api/v2/private/crypto_withdrawals`  List of withdrawals from the selected account. Includes all withdrawals: pending, executed, rejected etc. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateTradeHistory200Response**](rest_private_trade_history_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_eth_deposit_address

> models::RestPrivateCryptoDeposits200Response rest_private_slash_eth_deposit_address()
Ethereum deposit address

Exchange: `https://thalex.com/api/v2/private/eth_deposit_address`  Testnet: `https://testnet.thalex.com/api/v2/private/eth_deposit_address`  Get Ethereum deposit address

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestPrivateCryptoDeposits200Response**](rest_private_crypto_deposits_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_internal_transfer

> models::ErrorResponse rest_private_slash_internal_transfer(rest_private_internal_transfer_request)
Internal transfer

Exchange: `https://thalex.com/api/v2/private/internal_transfer`  Testnet: `https://testnet.thalex.com/api/v2/private/internal_transfer`  Transfer assets and/or positions from source (currently selected) to destination account.  Transfers are subject to margin checks. Please see `private/verify_internal_transfer` method description for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_internal_transfer_request** | Option<[**RestPrivateInternalTransferRequest**](RestPrivateInternalTransferRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_verify_internal_transfer

> models::RestPrivateVerifyInternalTransfer200Response rest_private_slash_verify_internal_transfer(rest_private_verify_internal_transfer_request)
Verify internal transfer

Exchange: `https://thalex.com/api/v2/private/verify_internal_transfer`  Testnet: `https://testnet.thalex.com/api/v2/private/verify_internal_transfer`  Verify if internal transfer of assets and/or positions from source (currently selected) to destination account is possible. Does not perform the transfer itself.  Transfers are subject to margin checks.  A transfer cannot result in an account breaching margin requirements. This applies to both source and destination accounts.  If either of the accounts is already in margin breach state, the transfer is only allowed if it results in an increase of available margin on that account. The other account must not breach margin requirement as a result of the transfer. This allows transferring assets and/or positions from an account with enough extra margin to an account that was margin called.  Each transfer can contain multiple asset and position transfers. It is checked for margin requirements as a single transaction. It is possible to specify negative amounts for transferred assets and positions, which results in a reverse direction of transfer (i.e. from destination account to the source one). This allows performing asset/position exchange operations, and is helpful when a leg of such operation alone would result in a margin requirements breach. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_verify_internal_transfer_request** | Option<[**RestPrivateVerifyInternalTransferRequest**](RestPrivateVerifyInternalTransferRequest.md)> |  |  |

### Return type

[**models::RestPrivateVerifyInternalTransfer200Response**](rest_private_verify_internal_transfer_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_verify_withdrawal

> models::RestPrivateInsert200Response rest_private_slash_verify_withdrawal(asset_name, amount, target_address)
Verify if withdrawal is possible

Exchange: `https://thalex.com/api/v2/private/verify_withdrawal`  Testnet: `https://testnet.thalex.com/api/v2/private/verify_withdrawal`  This method is subject to withdrawal permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_name** | **String** |  | [required] |
**amount** | **f64** |  | [required] |
**target_address** | **String** |  | [required] |

### Return type

[**models::RestPrivateInsert200Response**](rest_private_insert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_withdraw

> models::ErrorResponse rest_private_slash_withdraw(rest_private_withdraw_request)
Withdraw assets

Exchange: `https://thalex.com/api/v2/private/withdraw`  Testnet: `https://testnet.thalex.com/api/v2/private/withdraw`  This method is subject to withdrawal permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_withdraw_request** | Option<[**RestPrivateWithdrawRequest**](RestPrivateWithdrawRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

