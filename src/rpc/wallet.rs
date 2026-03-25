

use crate::{models::{
    VerifyWithdrawalRpcResult,
    WithdrawRpcResult,
    CryptoDepositsResult,
    CryptoDepositsResponse,
    OrderStatus,
    Ticker,
    WithdrawResult,
    CryptoDepositsRpcResult,
    EthDepositAddressResponse,
    VerifyInternalTransferResponse,
    VerifyInternalTransferResult,
    InternalTransferResult,
    WithdrawResponse,
    CryptoWithdrawalsResponse,
    InternalTransferParams,
    EthDepositAddressResult,
    VerifyWithdrawalResponse,
    InternalTransferResponse,
    CryptoWithdrawalsResult,
    VerifyInternalTransferParams,
    VerifyWithdrawalParams,
    BtcDepositAddressResult,
    WithdrawParams,
    VerifyInternalTransferRpcResult,
    CryptoWithdrawalsRpcResult,
    BtcDepositAddressRpcResult,
    BtcDepositAddressResponse,
    InternalTransferRpcResult,
    RpcErrorResponse,
    EthDepositAddressRpcResult,
    Index,
    Instrument,
    VerifyWithdrawalResult
}, ws_client::{
    WsClient,
}, types::{
    Error, ClientError,
}};
use serde_json::Value;
use rust_decimal::Decimal;

pub struct WalletRpc<'a> {
    pub client: &'a WsClient,
}
impl <'a> WalletRpc<'a> {

    /// Verify if withdrawal is possible
    /// returns: VerifyWithdrawalRpcResult
    pub async fn verify_withdrawal(&self, params: VerifyWithdrawalParams) -> Result<VerifyWithdrawalRpcResult, ClientError> {
        let result: Result<VerifyWithdrawalResponse, ClientError> = self
            .client
            .send_rpc(
                "private/verify_withdrawal",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    VerifyWithdrawalResponse::VerifyWithdrawalResult(res) => Ok(res.result),
                    VerifyWithdrawalResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Withdraw assets
    /// returns: WithdrawRpcResult
    pub async fn withdraw(&self, params: WithdrawParams) -> Result<WithdrawRpcResult, ClientError> {
        let result: Result<WithdrawResponse, ClientError> = self
            .client
            .send_rpc(
                "private/withdraw",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    WithdrawResponse::WithdrawResult(res) => Ok(res.result),
                    WithdrawResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Withdrawals
    /// returns: CryptoWithdrawalsRpcResult
    pub async fn crypto_withdrawals(&self, ) -> Result<CryptoWithdrawalsRpcResult, ClientError> {
        let result: Result<CryptoWithdrawalsResponse, ClientError> = self
            .client
            .send_rpc(
                "private/crypto_withdrawals",
                serde_json::to_value({}).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    CryptoWithdrawalsResponse::CryptoWithdrawalsResult(res) => Ok(res.result),
                    CryptoWithdrawalsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Deposits
    /// returns: CryptoDepositsRpcResult
    pub async fn crypto_deposits(&self, ) -> Result<CryptoDepositsRpcResult, ClientError> {
        let result: Result<CryptoDepositsResponse, ClientError> = self
            .client
            .send_rpc(
                "private/crypto_deposits",
                serde_json::to_value({}).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    CryptoDepositsResponse::CryptoDepositsResult(res) => Ok(res.result),
                    CryptoDepositsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Bitcoin deposit address
    /// returns: BtcDepositAddressRpcResult
    pub async fn btc_deposit_address(&self, ) -> Result<BtcDepositAddressRpcResult, ClientError> {
        let result: Result<BtcDepositAddressResponse, ClientError> = self
            .client
            .send_rpc(
                "private/btc_deposit_address",
                serde_json::to_value({}).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    BtcDepositAddressResponse::BtcDepositAddressResult(res) => Ok(res.result),
                    BtcDepositAddressResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Ethereum deposit address
    /// returns: EthDepositAddressRpcResult
    pub async fn eth_deposit_address(&self, ) -> Result<EthDepositAddressRpcResult, ClientError> {
        let result: Result<EthDepositAddressResponse, ClientError> = self
            .client
            .send_rpc(
                "private/eth_deposit_address",
                serde_json::to_value({}).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    EthDepositAddressResponse::EthDepositAddressResult(res) => Ok(res.result),
                    EthDepositAddressResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Verify internal transfer
    /// returns: VerifyInternalTransferRpcResult
    pub async fn verify_internal_transfer(&self, params: VerifyInternalTransferParams) -> Result<VerifyInternalTransferRpcResult, ClientError> {
        let result: Result<VerifyInternalTransferResponse, ClientError> = self
            .client
            .send_rpc(
                "private/verify_internal_transfer",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    VerifyInternalTransferResponse::VerifyInternalTransferResult(res) => Ok(res.result),
                    VerifyInternalTransferResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }


    /// Internal transfer
    /// returns: InternalTransferRpcResult
    pub async fn internal_transfer(&self, params: InternalTransferParams) -> Result<InternalTransferRpcResult, ClientError> {
        let result: Result<InternalTransferResponse, ClientError> = self
            .client
            .send_rpc(
                "private/internal_transfer",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    InternalTransferResponse::InternalTransferResult(res) => Ok(res.result),
                    InternalTransferResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }

    
}
