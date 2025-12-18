# ApiKeyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nickname** | **String** | Key nickname to be displayed in the front end. | 
**default_account_number** | **String** | Account number of the default account for this key. | 
**permissions** | [**Vec<models::ApiKeyConfigPermissionsInner>**](ApiKeyConfig_permissions_inner.md) | List of account permissions for this key. | 
**notifications** | Option<[**std::collections::HashMap<String, models::NotificationPreferencesApiKeyValue>**](NotificationPreferencesApiKey_value.md)> | Notification preferences per category | [optional]
**active** | **bool** | If `true` this API key is active, i.e. can be used to access the API. | 
**ip_whitelist** | Option<[**Vec<models::ApiKeyConfigIpWhitelistInner>**](ApiKeyConfig_ip_whitelist_inner.md)> | Whitelisted IP addresses.  If this list is not empty, the API key can only be used from one of the addresses on the list.  Each list item is either a single string that contains a specific IP address, or an array of exactly two strings that define a whitelisted range of IP addresses (range ends included). Strings must be valid representations of IPv4 or IPv6 addresses. Special addresses (broadcast, loopback, multicast etc.) are not allowed. The addresses must be public IPs as seen by our servers.  You can specify a maximum of 10 addresses.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


