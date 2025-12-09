# Success39ResultMark

## Enum Variants

| Name | Description |
|---- | -----|
| Vec<Vec<serde_json::Value>> | Array of mark price data points. Each mark price data point is an array of mark price data in OHLC format followed by an optional array of top of book data. Top of book data is returned only for &#x60;1m&#x60; resolution and set to &#x60;null&#x60; otherwise.  Note that the inner array format depends on the &#x60;instrument_type&#x60; flag.  Note also that the top bid and top ask data can be set to &#x60;null&#x60; for the &#x60;1m&#x60; resolution if there was no quote at the start of the correspondent minute interval.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


