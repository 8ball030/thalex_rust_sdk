# Success14ResultTransactionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | **f64** | Timestamp when the transaction was performed.  For actions that produce multiple transactions (e.g. asset swaps, internal transfers), all transactions will have the same timestamp.  | 
**asset** | **String** | Asset name. | 
**amount** | **f64** | Amount credited (positive number) or debited (negative number). | 
**instrument_name** | Option<**String**> | Instrument name this transaction relates to. For example, settlement transactions are per instrument.  Not included for transactions that don't relate to an instrument.  | [optional]
**transaction_type** | Option<**String**> | Transaction type. Can be one of the following values:  - `deposit` - Deposits and asset credits. - `withdrawal` - Withdrawals and asset debits. - `withdrawal fee` - Withdrawal fees. - `session settlement` - Settled session PNL. - `perpetual funding` - Settled perpetual funding. - `internal transfer` - Transfer of assets between sub-accounts. One transaction in each sub-account per asset per transfer. - `asset swap` - Swap between assets. One transaction for each side of the asset pair per swap. - `referral program payment` - Referral program rewards. - `market velocity program payment` - MVP program rewards. - `market quality program payment` - MQP program rewards. - `daily interest` - Daily penalty charge for negative balance. Not applied anymore, but can be found in historical transactions.  | [optional]
**description** | **String** | Description of this transaction.  Note that this field is not supposed to be machine-readable and the the format is not guaranteed to remain unchanged.  | 
**balance_after** | Option<**f64**> | Account balance in this asset right after transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


