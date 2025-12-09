# RestPrivateCreateRfqRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**legs** | [**Vec<models::RestPrivateCreateRfqRequestLegsInner>**](rest_private_create_rfq_request_legs_inner.md) | Specify any number of legs that you'd like to trade in a single package. Leg amounts may be positive (long) or negative (short), and must adhere to the regular volume tick size for the respective instrument. At least one leg must be long.  | 
**label** | Option<**String**> | User label for this RFQ, which will be reflected in eventual trades. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


