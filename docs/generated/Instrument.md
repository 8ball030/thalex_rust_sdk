# Instrument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument_name** | Option<**String**> |  | [optional]
**product** | Option<**String**> | E.g. \"FBTCUSD\", \"OBTCUSD\". | [optional]
**tick_size** | Option<**f64**> | Price alignment. | [optional]
**volume_tick_size** | Option<**f64**> | Amount alignment. | [optional]
**min_order_amount** | Option<**f64**> | Minimum order amount for this instrument. This value is always greater or equal to `volume_tick_size`.  If this value is greater than `volume_tick_size`, it is not possible to insert an order of a smaller amount, or amend an existing order to a smaller amount. However, orders in the books can have smaller remaining amounts as they get partially filled, down to the minimum of `volume_tick_size`.  | [optional]
**underlying** | Option<**String**> | Related index, e.g. \"BTCUSD\". | [optional]
**r#type** | Option<**String**> |  | [optional]
**option_type** | Option<**String**> |  | [optional]
**expiry_date** | Option<**String**> | Expiration date in ISO format (YYYY-mm-dd). | [optional]
**expiration_timestamp** | Option<**i32**> | Expiration time as Unix timestamp (seconds). | [optional]
**strike_price** | Option<**f64**> | Strike price of option. | [optional]
**base_currency** | Option<**String**> | Base currency for pricing (i.e. USD). | [optional]
**legs** | Option<[**Vec<models::InstrumentLegsInner>**](Instrument_legs_inner.md)> | For combinations, array of objects with `instrument_name` and `quantity`. | [optional]
**create_time** | Option<**f64**> | Creation time (Unix timestamp). | [optional]
**settlement_price** | Option<**f64**> | For expired instruments, the final settlement price. | [optional]
**settlement_index_price** | Option<**f64**> | For expired instruments, the underlying delivery price. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


