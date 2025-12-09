# Success16ResultCashInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | Currency name. | 
**balance** | **f64** | Current balance in this currency. | 
**collateral_factor** | **f64** | The collateral quality of the asset i.e. the fraction of the asset that can be used as a collateral. | 
**collateral_index_price** | **f64** | Index price used to calculate collateral effect of this position. Can be `null` for assets that are not converted using an index, e.g. for stable coins.  | 
**applied_collateral** | **f64** | Total collateral effect of this position. | 
**transactable** | **bool** | If this flag is `true`, this currency can be deposited and withdrawn. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


