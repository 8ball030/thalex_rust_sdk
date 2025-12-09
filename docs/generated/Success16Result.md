# Success16Result

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_number** | **String** | Account number | 
**unrealised_pnl** | **f64** | Total unrealised profit or loss. | 
**cash_collateral** | **f64** | Total margin based on cash holdings. | 
**margin** | **f64** | Total margin from unrealised P&L and cash holdings. | 
**required_margin** | **f64** | Required margin based on current position. | 
**remaining_margin** | **f64** | Difference between margin and required margin. | 
**session_realised_pnl** | **f64** | Total realised profit or loss in current session. | 
**realised_position_pnl** | **f64** | Position profit or loss in current session. | 
**realised_perpetual_funding** | **f64** | Realised perpetual funding profit or loss in current session. | 
**session_fees** | **f64** | Fees paid in current session. | 
**portfolio** | [**Vec<models::Success16ResultPortfolioInner>**](Success_16_result_portfolio_inner.md) | List of positions each with unrealised P&L. | 
**cash** | [**Vec<models::Success16ResultCashInner>**](Success_16_result_cash_inner.md) | List of cash holdings, for each relevant currency, and how they contribute to margin. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


