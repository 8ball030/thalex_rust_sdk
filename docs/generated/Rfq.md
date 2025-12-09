# Rfq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rfq_id** | **String** | Identifier for this RFQ. | 
**amount** | **f64** | Requested amount for this RFQ | 
**create_time** | **f64** | Timestamp of creation (unix timestamp) | 
**valid_until** | Option<**f64**> | Timestamp at which this RFQ will be automatically cancelled (unix timestamp) | [optional]
**legs** | [**Vec<models::RfqLegsInner>**](Rfq_legs_inner.md) | Combo legs. A minimalised version of the request such that quantities are integer. | 
**label** | Option<**String**> | User label set at creation. Not visibible to market makers. | [optional]
**insert_reason** | Option<**String**> | Detailed reason for creation. Visible only to the requester. | [optional]
**delete_reason** | Option<**String**> | Reason why this RFQ was removed. Visible only to the requester. | [optional]
**volume_tick_size** | Option<**f64**> | The minimum size / size increase for quotes. | [optional]
**quoted_bid** | Option<[**models::RfqQuotedBid**](Rfq_quoted_bid.md)> |  | [optional]
**quoted_ask** | Option<[**models::RfqQuotedAsk**](Rfq_quoted_ask.md)> |  | [optional]
**trade_price** | Option<**f64**> | Combo price for which this RFQ was traded. | [optional]
**trade_amount** | Option<**f64**> | Amount in which this RFQ was traded. | [optional]
**close_time** | Option<**f64**> | Time at which this RFQ was cancelled or traded. | [optional]
**event** | Option<**String**> | This field is set only on the `account.rfqs` subscription. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


