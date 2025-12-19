# Ocq1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | **String** | Individually identifies this bot instance. You can use it to cancel this specific one. | 
**status** | **String** |  | 
**stop_reason** | Option<**String**> | The reason why the bot stopped executing. Possible values are [\"client_cancel\", \"client_bulk_cancel\", \"end_time\", \"instrument_deactivated\", \"margin_breach\", \"admin_cancel\", \"conflict\", \"strategy\"]. | [optional]
**strategy** | **String** | equal to \"ocq\" | 
**instrument_name** | **String** | Name of the instrument. | 
**signal** | **String** |  | 
**bid_offset** | **f64** | The offset of the price of the bid quote from the signal price. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**ask_offset** | **f64** | The offset of the price of the ask quote from the signal price. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**quote_size** | **f64** | The default size of both the bid and ask quote, See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**min_position** | **f64** | The minimum portfolio position to maintain in the subaccount. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**max_position** | **f64** | The maximum portfolio position to maintain in the subaccount. See [the bot strategies section](#tag/bot_strategies) for more explanation. | 
**end_time** | **f64** | Timestamp when the bot stops executing, cancelling its orders and leaving all positions of the subaccount intact. | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


