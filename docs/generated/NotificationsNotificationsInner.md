# NotificationsNotificationsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique notification ID. | 
**time** | **f64** | Time of notification (Unix timestamp). | 
**category** | **String** | Notification category (see API description / Notifications). | 
**title** | **String** | Human-readable title for the notification. | 
**message** | **String** | Human-readable message for the notification. | 
**display_type** | **String** | Specifies what style to use for notification display. | 
**read** | **bool** | set to true if notification was marked as read. | 
**account_name** | Option<**String**> | Optional account name, only present if the notification is related to an account. | [optional]
**account_number** | Option<**String**> | Optional account number only present if the notification is related to an account. | [optional]
**popup** | **bool** | User preference - show popup for this notification | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


