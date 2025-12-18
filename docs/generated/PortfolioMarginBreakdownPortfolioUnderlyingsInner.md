# PortfolioMarginBreakdownPortfolioUnderlyingsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**underlying** | Option<**String**> | Underlying index name. | [optional]
**required_margin** | Option<**f64**> | Total margin required for positions with this underlying. | [optional]
**scenario_margin** | Option<**f64**> | Deprecated. Same value as `loss_margin`. | [optional]
**loss_margin** | Option<**f64**> | Margin based on scenario loss coverage. | [optional]
**d1_roll_cash_position** | Option<**f64**> | Delta one roll position x index price in this underlying. | [optional]
**options_roll_cash_position** | Option<**f64**> | Options roll position x index price in this underlying. | [optional]
**roll_cash_position** | Option<**f64**> | Roll position x index price in this underlying. | [optional]
**d1_roll_contingency_margin** | Option<**f64**> | Margin based on delta one roll position. | [optional]
**options_roll_contingency_margin** | Option<**f64**> | Margin based on options roll position. | [optional]
**roll_contingency_margin** | Option<**f64**> | Margin based on roll position. | [optional]
**options_short_cash_position** | Option<**f64**> | Total short options position x index price in this underlying. | [optional]
**options_contingency_margin** | Option<**f64**> | Margin based on options short position. | [optional]
**scenario_used** | Option<**i32**> | Index of the scenario in the `scenarios` array that was used to calculate total margin requirements for this underlying. | [optional]
**scenarios** | Option<[**Vec<models::PortfolioMarginBreakdownPortfolioUnderlyingsInnerScenariosInner>**](PortfolioMarginBreakdown_portfolio_underlyings_inner_scenarios_inner.md)> | Scenarios that were simulated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


