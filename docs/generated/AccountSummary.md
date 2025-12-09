# AccountSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unrealised_pnl** | **f64** | Total unrealised profit or loss. | 
**cash_collateral** | **f64** | Total margin based on cash holdings. | 
**margin** | **f64** | Total margin from unrealised P&L and cash holdings. | 
**required_margin** | **f64** | Required margin based on current position. | 
**remaining_margin** | **f64** | Difference between margin and required margin. | 
**session_realised_pnl** | **f64** | Realised profit or loss in current session. | 
**cash** | [**Vec<models::AccountSummaryCashInner>**](AccountSummary_cash_inner.md) | List of cash holdings, for each relevant currency, and how they contribute to margin. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


