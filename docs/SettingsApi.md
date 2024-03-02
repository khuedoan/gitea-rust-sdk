# Gitea\SettingsApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_general_api_settings**](SettingsApi.md#get_general_api_settings) | **GET** /settings/api | Get instance's global settings for api
[**get_general_attachment_settings**](SettingsApi.md#get_general_attachment_settings) | **GET** /settings/attachment | Get instance's global settings for Attachment
[**get_general_repository_settings**](SettingsApi.md#get_general_repository_settings) | **GET** /settings/repository | Get instance's global settings for repositories
[**get_general_ui_settings**](SettingsApi.md#get_general_ui_settings) | **GET** /settings/ui | Get instance's global settings for ui



## get_general_api_settings

> models::GeneralApiSettings get_general_api_settings()
Get instance's global settings for api

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GeneralApiSettings**](GeneralAPISettings.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_attachment_settings

> models::GeneralAttachmentSettings get_general_attachment_settings()
Get instance's global settings for Attachment

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GeneralAttachmentSettings**](GeneralAttachmentSettings.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_repository_settings

> models::GeneralRepoSettings get_general_repository_settings()
Get instance's global settings for repositories

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GeneralRepoSettings**](GeneralRepoSettings.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_ui_settings

> models::GeneralUiSettings get_general_ui_settings()
Get instance's global settings for ui

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GeneralUiSettings**](GeneralUISettings.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

