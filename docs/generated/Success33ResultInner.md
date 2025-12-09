# Success33ResultInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | Withdrawn currency symbol. | 
**amount** | **f64** | Amount of currency withdrawn. | 
**target_address** | **String** | Target address, specific to blockchain used. | 
**blockchain** | Option<**String**> | Blockchain used or this transaction. | [optional]
**transaction_hash** | Option<**String**> | Transaction hash on the used blockchain. | [optional]
**create_time** | **f64** | Time when this withdrawal was requested (Unix timestamp). | 
**label** | Option<**String**> | Optional label attached to the withdrawal request. | [optional]
**state** | **String** | Withdrawal transaction status. | 
**remark** | Option<**String**> | Remark added by the exchange. | [optional]
**fee** | Option<**f64**> | Amount of fees withheld. | [optional]
**fee_asset** | Option<**String**> | Asset in which the withdrawal fees are withheld. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


