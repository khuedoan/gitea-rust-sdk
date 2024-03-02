# Gitea\NotificationApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notify_get_list**](NotificationApi.md#notify_get_list) | **GET** /notifications | List users's notification threads
[**notify_get_repo_list**](NotificationApi.md#notify_get_repo_list) | **GET** /repos/{owner}/{repo}/notifications | List users's notification threads on a specific repo
[**notify_get_thread**](NotificationApi.md#notify_get_thread) | **GET** /notifications/threads/{id} | Get notification thread by ID
[**notify_new_available**](NotificationApi.md#notify_new_available) | **GET** /notifications/new | Check if unread notifications exist
[**notify_read_list**](NotificationApi.md#notify_read_list) | **PUT** /notifications | Mark notification threads as read, pinned or unread
[**notify_read_repo_list**](NotificationApi.md#notify_read_repo_list) | **PUT** /repos/{owner}/{repo}/notifications | Mark notification threads as read, pinned or unread on a specific repo
[**notify_read_thread**](NotificationApi.md#notify_read_thread) | **PATCH** /notifications/threads/{id} | Mark notification thread as read by ID



## notify_get_list

> Vec<models::NotificationThread> notify_get_list(all, status_types, subject_type, since, before, page, limit)
List users's notification threads

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | If true, show notifications marked as read. Default value is false |  |
**status_types** | Option<[**Vec<String>**](String.md)> | Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned. |  |
**subject_type** | Option<[**Vec<String>**](String.md)> | filter notifications by subject type |  |
**since** | Option<**String**> | Only show notifications updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show notifications updated before the given time. This is a timestamp in RFC 3339 format |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::NotificationThread>**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_get_repo_list

> Vec<models::NotificationThread> notify_get_repo_list(owner, repo, all, status_types, subject_type, since, before, page, limit)
List users's notification threads on a specific repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**all** | Option<**bool**> | If true, show notifications marked as read. Default value is false |  |
**status_types** | Option<[**Vec<String>**](String.md)> | Show notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread & pinned |  |
**subject_type** | Option<[**Vec<String>**](String.md)> | filter notifications by subject type |  |
**since** | Option<**String**> | Only show notifications updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show notifications updated before the given time. This is a timestamp in RFC 3339 format |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::NotificationThread>**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_get_thread

> models::NotificationThread notify_get_thread(id)
Get notification thread by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of notification thread | [required] |

### Return type

[**models::NotificationThread**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_new_available

> models::NotificationCount notify_new_available()
Check if unread notifications exist

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NotificationCount**](NotificationCount.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_read_list

> Vec<models::NotificationThread> notify_read_list(last_read_at, all, status_types, to_status)
Mark notification threads as read, pinned or unread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_read_at** | Option<**String**> | Describes the last point that notifications were checked. Anything updated since this time will not be updated. |  |
**all** | Option<**String**> | If true, mark all notifications on this repo. Default value is false |  |
**status_types** | Option<[**Vec<String>**](String.md)> | Mark notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread. |  |
**to_status** | Option<**String**> | Status to mark notifications as, Defaults to read. |  |

### Return type

[**Vec<models::NotificationThread>**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_read_repo_list

> Vec<models::NotificationThread> notify_read_repo_list(owner, repo, all, status_types, to_status, last_read_at)
Mark notification threads as read, pinned or unread on a specific repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**all** | Option<**String**> | If true, mark all notifications on this repo. Default value is false |  |
**status_types** | Option<[**Vec<String>**](String.md)> | Mark notifications with the provided status types. Options are: unread, read and/or pinned. Defaults to unread. |  |
**to_status** | Option<**String**> | Status to mark notifications as. Defaults to read. |  |
**last_read_at** | Option<**String**> | Describes the last point that notifications were checked. Anything updated since this time will not be updated. |  |

### Return type

[**Vec<models::NotificationThread>**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_read_thread

> models::NotificationThread notify_read_thread(id, to_status)
Mark notification thread as read by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of notification thread | [required] |
**to_status** | Option<**String**> | Status to mark notifications as |  |[default to read]

### Return type

[**models::NotificationThread**](NotificationThread.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

