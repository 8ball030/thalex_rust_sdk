# \NotificationsApi

All URIs are relative to *https://thalex.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rest_private_slash_mark_inbox_notification_as_read**](NotificationsApi.md#rest_private_slash_mark_inbox_notification_as_read) | **POST** /private/mark_inbox_notification_as_read | Marking notification as read
[**rest_private_slash_notifications_inbox**](NotificationsApi.md#rest_private_slash_notifications_inbox) | **GET** /private/notifications_inbox | Notifications inbox



## rest_private_slash_mark_inbox_notification_as_read

> models::ErrorResponse rest_private_slash_mark_inbox_notification_as_read(rest_private_mark_inbox_notification_as_read_request)
Marking notification as read

Exchange: `https://thalex.com/api/v2/private/mark_inbox_notification_as_read`  Testnet: `https://testnet.thalex.com/api/v2/private/mark_inbox_notification_as_read`  Mark a notification as read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_private_mark_inbox_notification_as_read_request** | Option<[**RestPrivateMarkInboxNotificationAsReadRequest**](RestPrivateMarkInboxNotificationAsReadRequest.md)> |  |  |

### Return type

[**models::ErrorResponse**](ErrorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_private_slash_notifications_inbox

> models::RestPrivateNotificationsInbox200Response rest_private_slash_notifications_inbox(limit)
Notifications inbox

Exchange: `https://thalex.com/api/v2/private/notifications_inbox`  Testnet: `https://testnet.thalex.com/api/v2/private/notifications_inbox`  This method returns a list of latest notifications that were sent to the current user. The list only contains items for which the `inbox` preference is set` (either in user preferences, or by default). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**models::RestPrivateNotificationsInbox200Response**](rest_private_notifications_inbox_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

