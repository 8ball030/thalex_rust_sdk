# Ocq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strategy** | **String** | Equal to \"ocq\". | 
**instrument_name** | **String** | Name of the instrument to run the One Click Quoter on. | 
**signal** | **String** |  | 
**bid_offset** | **f64** | The offset of the price of the bid quote from the signal price. Must be smaller than `ask_offset`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**ask_offset** | **f64** | The offset of the price of the ask quote from the signal price. Must be greater than `bid_offset`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**exit_offset** | Option<**f64**> | Optional offset of the price of the exit quote from the signal price. Must be between `bid_offset` and `ask_offset`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | [optional]
**quote_size** | **f64** | The default size of both the bid and ask quote. Must not be greater than 250 volume ticks. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**min_position** | **f64** | The minimum portfolio position to maintain in the subaccount. Must be smaller than `max_position`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**max_position** | **f64** | The maximum portfolio position to maintain in the subaccount. Must be greater than `min_position`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | 
**target_position** | Option<**f64**> | Optional portfolio position to maintain in the subaccount. Must be between `min_position` and `max_position`. See [the bot strategies section](#tag/bot_strategies) for more explanation.  | [optional]
**end_time** | **f64** | Timestamp when the bot should stop executing. Must not be further away than a week. When `end_time` is reached, the bot will leave all positions intact, it will not open/close any of them.  | 
**label** | Option<**String**> | A label that the bot will add to all orders for easy identification. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


