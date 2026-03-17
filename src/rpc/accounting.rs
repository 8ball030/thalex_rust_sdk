use crate::{
    models::{
        AccountBreakdownResponse, AccountBreakdownRpcResult, AccountSummary,
        AccountSummaryResponse, ConditionalOrderHistoryParams, ConditionalOrderHistoryResponse,
        ConditionalOrderHistoryRpcResult, DailyMarkHistoryParams, DailyMarkHistoryResponse,
        DailyMarkHistoryRpcResult, MarginBreakdownWithOrder, OpenOrdersResponse,
        OrderHistoryParams, OrderHistoryResponse, OrderHistoryRpcResult, OrderStatus,
        PortfolioEntry, PortfolioMarginBreakdown, PortfolioResponse,
        RequiredMarginBreakdownResponse, RequiredMarginForOrderParams,
        RequiredMarginForOrderResponse, RfqHistoryParams, RfqHistoryResponse, RfqHistoryRpcResult,
        TradeHistoryParams, TradeHistoryResponse, TradeHistoryRpcResult, TradeValueHistoryParams,
        TradeValueHistoryResponse, TradeValueHistoryRpcResult, TransactionHistoryParams,
        TransactionHistoryResponse, TransactionHistoryRpcResult,
    },
    types::ClientError,
    ws_client::WsClient,
};

pub struct AccountingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> AccountingRpc<'a> {
    /// Portfolio
    /// returns: Vec<PortfolioEntry>
    pub async fn portfolio(&self) -> Result<Vec<PortfolioEntry>, ClientError> {
        let result: Result<PortfolioResponse, ClientError> = self
            .client
            .send_rpc(
                "private/portfolio",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                PortfolioResponse::PortfolioResult(res) => Ok(res.result),
                PortfolioResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Open orders
    /// returns: Vec<OrderStatus>
    pub async fn open_orders(&self) -> Result<Vec<OrderStatus>, ClientError> {
        let result: Result<OpenOrdersResponse, ClientError> = self
            .client
            .send_rpc(
                "private/open_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                OpenOrdersResponse::OpenOrdersResult(res) => Ok(res.result),
                OpenOrdersResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Order history
    /// returns: OrderHistoryRpcResult
    pub async fn order_history(
        &self,
        params: OrderHistoryParams,
    ) -> Result<OrderHistoryRpcResult, ClientError> {
        let result: Result<OrderHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/order_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                OrderHistoryResponse::OrderHistoryResult(res) => Ok(res.result),
                OrderHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Conditional order history
    /// returns: ConditionalOrderHistoryRpcResult
    pub async fn conditional_order_history(
        &self,
        params: ConditionalOrderHistoryParams,
    ) -> Result<ConditionalOrderHistoryRpcResult, ClientError> {
        let result: Result<ConditionalOrderHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/conditional_order_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                ConditionalOrderHistoryResponse::ConditionalOrderHistoryResult(res) => {
                    Ok(res.result)
                }
                ConditionalOrderHistoryResponse::RpcErrorResponse(err) => {
                    Err(ClientError::Rpc(err))
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Trade history
    /// returns: TradeHistoryRpcResult
    pub async fn trade_history(
        &self,
        params: TradeHistoryParams,
    ) -> Result<TradeHistoryRpcResult, ClientError> {
        let result: Result<TradeHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/trade_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                TradeHistoryResponse::TradeHistoryResult(res) => Ok(res.result),
                TradeHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Trading volume historical data.
    /// returns: TradeValueHistoryRpcResult
    pub async fn trade_value_history(
        &self,
        params: TradeValueHistoryParams,
    ) -> Result<TradeValueHistoryRpcResult, ClientError> {
        let result: Result<TradeValueHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/trade_value_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                TradeValueHistoryResponse::TradeValueHistoryResult(res) => Ok(res.result),
                TradeValueHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Daily mark history
    /// returns: DailyMarkHistoryRpcResult
    pub async fn daily_mark_history(
        &self,
        params: DailyMarkHistoryParams,
    ) -> Result<DailyMarkHistoryRpcResult, ClientError> {
        let result: Result<DailyMarkHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/daily_mark_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                DailyMarkHistoryResponse::DailyMarkHistoryResult(res) => Ok(res.result),
                DailyMarkHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Transaction history
    /// returns: TransactionHistoryRpcResult
    pub async fn transaction_history(
        &self,
        params: TransactionHistoryParams,
    ) -> Result<TransactionHistoryRpcResult, ClientError> {
        let result: Result<TransactionHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/transaction_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                TransactionHistoryResponse::TransactionHistoryResult(res) => Ok(res.result),
                TransactionHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// RFQ history
    /// returns: RfqHistoryRpcResult
    pub async fn rfq_history(
        &self,
        params: RfqHistoryParams,
    ) -> Result<RfqHistoryRpcResult, ClientError> {
        let result: Result<RfqHistoryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/rfq_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                RfqHistoryResponse::RfqHistoryResult(res) => Ok(res.result),
                RfqHistoryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Account breakdown
    /// returns: AccountBreakdownRpcResult
    pub async fn account_breakdown(&self) -> Result<AccountBreakdownRpcResult, ClientError> {
        let result: Result<AccountBreakdownResponse, ClientError> = self
            .client
            .send_rpc(
                "private/account_breakdown",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                AccountBreakdownResponse::AccountBreakdownResult(res) => Ok(res.result),
                AccountBreakdownResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Account summary
    /// returns: AccountSummary
    pub async fn account_summary(&self) -> Result<AccountSummary, ClientError> {
        let result: Result<AccountSummaryResponse, ClientError> = self
            .client
            .send_rpc(
                "private/account_summary",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                AccountSummaryResponse::AccountSummaryResult(res) => Ok(res.result),
                AccountSummaryResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Margin breakdown
    /// returns: PortfolioMarginBreakdown
    pub async fn required_margin_breakdown(&self) -> Result<PortfolioMarginBreakdown, ClientError> {
        let result: Result<RequiredMarginBreakdownResponse, ClientError> = self
            .client
            .send_rpc(
                "private/required_margin_breakdown",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                RequiredMarginBreakdownResponse::RequiredMarginBreakdownResult(res) => {
                    Ok(res.result)
                }
                RequiredMarginBreakdownResponse::RpcErrorResponse(err) => {
                    Err(ClientError::Rpc(err))
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Margin breakdown with order
    /// returns: MarginBreakdownWithOrder
    pub async fn required_margin_for_order(
        &self,
        params: RequiredMarginForOrderParams,
    ) -> Result<MarginBreakdownWithOrder, ClientError> {
        let result: Result<RequiredMarginForOrderResponse, ClientError> = self
            .client
            .send_rpc(
                "private/required_margin_for_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                RequiredMarginForOrderResponse::RequiredMarginForOrderResult(res) => Ok(res.result),
                RequiredMarginForOrderResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
