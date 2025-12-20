# SessionInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product** | Option<**String**> |  | [optional]
**remaining_amount** | Option<**f64**> | Remaining amount may be less than zero if a single aggressive order has caused an overshoot. If remaining amount is less than or equal to zero, the mass quote orders in the group will have been cancelled, and the group needs refilling.  | [optional]
**reason** | Option<**String**> | Detailed reason of the protection group status update.  The following reasons are possible: - `refill`: Remaining amount was updated by the client, e.g. with a call to `private/set_mm_protection`.  - `executions`: Remaining amount was updated after a trade.  - `client_cancel`: Protection group was deleted by the client, e.g. with a call to `private/cancel_mass_quote`.  - `session_end`: Protection group was automatically deleted when a WebSocket session ended.  - `failover`: Protection group was automatically deleted on matching engine failover.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


