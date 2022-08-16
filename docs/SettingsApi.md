# \SettingsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_general_api_settings**](SettingsApi.md#get_general_api_settings) | **GET** /settings/api | Get instance's global settings for api
[**get_general_attachment_settings**](SettingsApi.md#get_general_attachment_settings) | **GET** /settings/attachment | Get instance's global settings for Attachment
[**get_general_repository_settings**](SettingsApi.md#get_general_repository_settings) | **GET** /settings/repository | Get instance's global settings for repositories
[**get_general_ui_settings**](SettingsApi.md#get_general_ui_settings) | **GET** /settings/ui | Get instance's global settings for ui



## get_general_api_settings

> crate::models::GeneralApiSettings get_general_api_settings()
Get instance's global settings for api

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GeneralApiSettings**](GeneralAPISettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_attachment_settings

> crate::models::GeneralAttachmentSettings get_general_attachment_settings()
Get instance's global settings for Attachment

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GeneralAttachmentSettings**](GeneralAttachmentSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_repository_settings

> crate::models::GeneralRepoSettings get_general_repository_settings()
Get instance's global settings for repositories

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GeneralRepoSettings**](GeneralRepoSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_ui_settings

> crate::models::GeneralUiSettings get_general_ui_settings()
Get instance's global settings for ui

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GeneralUiSettings**](GeneralUISettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

