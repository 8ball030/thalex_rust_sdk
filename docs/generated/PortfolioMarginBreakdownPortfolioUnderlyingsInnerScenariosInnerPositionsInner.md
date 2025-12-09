# PortfolioMarginBreakdownPortfolioUnderlyingsInnerScenariosInnerPositionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument_name** | Option<**String**> | Instrument feedcode. | [optional]
**position** | Option<**f64**> | Position/order size in contracts. | [optional]
**instrument_pnl** | Option<**f64**> | P&L for a single contract of this instrument. | [optional]
**pnl** | Option<**f64**> | Total P&L for the position/order in this instrument. | [optional]
**current_price** | Option<**f64**> | The price at the moment of scenario calculation.  For positions this is set to the current mark price.  For open orders this is the order limit price.  | [optional]
**scenario_price** | Option<**f64**> | The price simulated for the scenario. | [optional]
**open_order** | Option<**bool**> | Indicates whether this position is implied from an open order. | [optional]
**assumed_filled** | Option<**bool**> | Indicates whether this order was assumed to be filled in this scenario.  Orders are assumed to be filled if they, for the particular scenario, generate an immediate loss if filled at the limit price.  Only orders that are assumed to be filled are taken into account for the loss, roll contingency and option contingency margin requirements.  This field is only present if `open_order` is `true`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


