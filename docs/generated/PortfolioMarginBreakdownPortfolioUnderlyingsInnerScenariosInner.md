# PortfolioMarginBreakdownPortfolioUnderlyingsInnerScenariosInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**underlying_change_pct** | Option<**f64**> | Simulated underlying change, % | [optional]
**vol_change_pct_point** | Option<**f64**> | Simulated volatility change, % | [optional]
**pnl** | Option<**f64**> | P&L resulting from this scenario. | [optional]
**coverage_factor** | Option<**f64**> | Effect factor of P&L on the loss margin. | [optional]
**required_margin** | Option<**f64**> | Total required margin in this scenario. | [optional]
**loss_margin** | Option<**f64**> | Margin based on scenario loss coverage. | [optional]
**roll_cash_position** | Option<**f64**> | Roll position x index price in this underlying. | [optional]
**roll_contingency_margin** | Option<**f64**> | Margin based on roll position. | [optional]
**d1_roll_cash_position** | Option<**f64**> | Delta one roll position x index price in this underlying. | [optional]
**d1_roll_contingency_margin** | Option<**f64**> | Margin based on delta one roll position. | [optional]
**options_roll_cash_position** | Option<**f64**> | Options roll position x index price in this underlying. | [optional]
**options_roll_contingency_margin** | Option<**f64**> | Margin based on options roll position. | [optional]
**options_short_cash_position** | Option<**f64**> | Total short options position x index price in this underlying. | [optional]
**options_contingency_margin** | Option<**f64**> | Margin based on options short position. | [optional]
**positions** | Option<[**Vec<models::PortfolioMarginBreakdownPortfolioUnderlyingsInnerScenariosInnerPositionsInner>**](PortfolioMarginBreakdown_portfolio_underlyings_inner_scenarios_inner_positions_inner.md)> |  | [optional]
**assets** | Option<[**Vec<models::PortfolioMarginBreakdownPortfolioUnderlyingsInnerScenariosInnerAssetsInner>**](PortfolioMarginBreakdown_portfolio_underlyings_inner_scenarios_inner_assets_inner.md)> | Results of scenario simulations on affected asset positions. | [optional]
**highlight** | Option<**bool**> | `true` if this scenario was used to calculate total margin requirements for this underlying. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


