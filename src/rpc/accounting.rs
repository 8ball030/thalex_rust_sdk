use crate::{
    models::{
        AccountBreakdownResponse, AccountBreakdownRpcResult, AccountSummary,
        AccountSummaryResponse, DailyMarkHistoryParams, DailyMarkHistoryResponse,
        DailyMarkHistoryRpcResult, MarginBreakdownWithOrder, OpenOrdersResponse,
        OrderHistoryParams, OrderHistoryResponse, OrderHistoryRpcResult, OrderStatus,
        PortfolioEntry, PortfolioMarginBreakdown, PortfolioResponse,
        RequiredMarginBreakdownResponse, RequiredMarginForOrderParams,
        RequiredMarginForOrderResponse, RfqHistoryParams, RfqHistoryResponse, RfqHistoryRpcResult,
        RpcErrorResponse, TradeHistoryParams, TradeHistoryResponse, TradeHistoryRpcResult,
        TransactionHistoryParams, TransactionHistoryResponse, TransactionHistoryRpcResult,
    },
    ws_client::WsClient,
};

pub struct AccountingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> AccountingRpc<'a> {
    /// Portfolio
    /// returns: Vec<PortfolioEntry>
    pub async fn portfolio(&self) -> Result<Vec<PortfolioEntry>, RpcErrorResponse> {
        let result: PortfolioResponse = self
            .client
            .send_rpc(
                "private/portfolio",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            PortfolioResponse::PortfolioResult(res) => Ok(res.result),
            PortfolioResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Open orders
    /// returns: Vec<OrderStatus>
    pub async fn open_orders(&self) -> Result<Vec<OrderStatus>, RpcErrorResponse> {
        let result: OpenOrdersResponse = self
            .client
            .send_rpc(
                "private/open_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            OpenOrdersResponse::OpenOrdersResult(res) => Ok(res.result),
            OpenOrdersResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Order history
    /// returns: OrderHistoryRpcResult
    pub async fn order_history(
        &self,
        params: OrderHistoryParams,
    ) -> Result<OrderHistoryRpcResult, RpcErrorResponse> {
        let result: OrderHistoryResponse = self
            .client
            .send_rpc(
                "private/order_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            OrderHistoryResponse::OrderHistoryResult(res) => Ok(res.result),
            OrderHistoryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Trade history
    /// returns: TradeHistoryRpcResult
    pub async fn trade_history(
        &self,
        params: TradeHistoryParams,
    ) -> Result<TradeHistoryRpcResult, RpcErrorResponse> {
        let result: TradeHistoryResponse = self
            .client
            .send_rpc(
                "private/trade_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            TradeHistoryResponse::TradeHistoryResult(res) => Ok(res.result),
            TradeHistoryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Daily mark history
    /// returns: DailyMarkHistoryRpcResult
    pub async fn daily_mark_history(
        &self,
        params: DailyMarkHistoryParams,
    ) -> Result<DailyMarkHistoryRpcResult, RpcErrorResponse> {
        let result: DailyMarkHistoryResponse = self
            .client
            .send_rpc(
                "private/daily_mark_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            DailyMarkHistoryResponse::DailyMarkHistoryResult(res) => Ok(res.result),
            DailyMarkHistoryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Transaction history
    /// returns: TransactionHistoryRpcResult
    pub async fn transaction_history(
        &self,
        params: TransactionHistoryParams,
    ) -> Result<TransactionHistoryRpcResult, RpcErrorResponse> {
        let result: TransactionHistoryResponse = self
            .client
            .send_rpc(
                "private/transaction_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            TransactionHistoryResponse::TransactionHistoryResult(res) => Ok(res.result),
            TransactionHistoryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// RFQ history
    /// returns: RfqHistoryRpcResult
    pub async fn rfq_history(
        &self,
        params: RfqHistoryParams,
    ) -> Result<RfqHistoryRpcResult, RpcErrorResponse> {
        let result: RfqHistoryResponse = self
            .client
            .send_rpc(
                "private/rfq_history",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            RfqHistoryResponse::RfqHistoryResult(res) => Ok(res.result),
            RfqHistoryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Account breakdown
    /// returns: AccountBreakdownRpcResult
    pub async fn account_breakdown(&self) -> Result<AccountBreakdownRpcResult, RpcErrorResponse> {
        let result: AccountBreakdownResponse = self
            .client
            .send_rpc(
                "private/account_breakdown",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            AccountBreakdownResponse::AccountBreakdownResult(res) => Ok(res.result),
            AccountBreakdownResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Account summary
    /// returns: AccountSummary
    pub async fn account_summary(&self) -> Result<AccountSummary, RpcErrorResponse> {
        let result: AccountSummaryResponse = self
            .client
            .send_rpc(
                "private/account_summary",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            AccountSummaryResponse::AccountSummaryResult(res) => Ok(res.result),
            AccountSummaryResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Margin breakdown
    /// returns: PortfolioMarginBreakdown
    pub async fn required_margin_breakdown(
        &self,
    ) -> Result<PortfolioMarginBreakdown, RpcErrorResponse> {
        let result: RequiredMarginBreakdownResponse = self
            .client
            .send_rpc(
                "private/required_margin_breakdown",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            RequiredMarginBreakdownResponse::RequiredMarginBreakdownResult(res) => Ok(res.result),
            RequiredMarginBreakdownResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Margin breakdown with order
    /// returns: MarginBreakdownWithOrder
    pub async fn required_margin_for_order(
        &self,
        params: RequiredMarginForOrderParams,
    ) -> Result<MarginBreakdownWithOrder, RpcErrorResponse> {
        let result: RequiredMarginForOrderResponse = self
            .client
            .send_rpc(
                "private/required_margin_for_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            RequiredMarginForOrderResponse::RequiredMarginForOrderResult(res) => Ok(res.result),
            RequiredMarginForOrderResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
