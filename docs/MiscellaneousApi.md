# Gitea\MiscellaneousApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gitignore_template_info**](MiscellaneousApi.md#get_gitignore_template_info) | **GET** /gitignore/templates/{name} | Returns information about a gitignore template
[**get_label_template_info**](MiscellaneousApi.md#get_label_template_info) | **GET** /label/templates/{name} | Returns all labels in a template
[**get_license_template_info**](MiscellaneousApi.md#get_license_template_info) | **GET** /licenses/{name} | Returns information about a license template
[**get_node_info**](MiscellaneousApi.md#get_node_info) | **GET** /nodeinfo | Returns the nodeinfo of the Gitea application
[**get_signing_key**](MiscellaneousApi.md#get_signing_key) | **GET** /signing-key.gpg | Get default signing-key.gpg
[**get_version**](MiscellaneousApi.md#get_version) | **GET** /version | Returns the version of the Gitea application
[**list_gitignores_templates**](MiscellaneousApi.md#list_gitignores_templates) | **GET** /gitignore/templates | Returns a list of all gitignore templates
[**list_label_templates**](MiscellaneousApi.md#list_label_templates) | **GET** /label/templates | Returns a list of all label templates
[**list_license_templates**](MiscellaneousApi.md#list_license_templates) | **GET** /licenses | Returns a list of all license templates
[**render_markdown**](MiscellaneousApi.md#render_markdown) | **POST** /markdown | Render a markdown document as HTML
[**render_markdown_raw**](MiscellaneousApi.md#render_markdown_raw) | **POST** /markdown/raw | Render raw markdown as HTML
[**render_markup**](MiscellaneousApi.md#render_markup) | **POST** /markup | Render a markup document as HTML



## get_gitignore_template_info

> models::GitignoreTemplateInfo get_gitignore_template_info(name)
Returns information about a gitignore template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the template | [required] |

### Return type

[**models::GitignoreTemplateInfo**](GitignoreTemplateInfo.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_label_template_info

> Vec<models::LabelTemplate> get_label_template_info(name)
Returns all labels in a template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the template | [required] |

### Return type

[**Vec<models::LabelTemplate>**](LabelTemplate.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_template_info

> models::LicenseTemplateInfo get_license_template_info(name)
Returns information about a license template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the license | [required] |

### Return type

[**models::LicenseTemplateInfo**](LicenseTemplateInfo.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_info

> models::NodeInfo get_node_info()
Returns the nodeinfo of the Gitea application

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NodeInfo**](NodeInfo.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signing_key

> String get_signing_key()
Get default signing-key.gpg

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> models::ServerVersion get_version()
Returns the version of the Gitea application

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerVersion**](ServerVersion.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gitignores_templates

> Vec<String> list_gitignores_templates()
Returns a list of all gitignore templates

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_label_templates

> Vec<String> list_label_templates()
Returns a list of all label templates

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_license_templates

> Vec<models::LicensesTemplateListEntry> list_license_templates()
Returns a list of all license templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LicensesTemplateListEntry>**](LicensesTemplateListEntry.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_markdown

> String render_markdown(body)
Render a markdown document as HTML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**MarkdownOption**](MarkdownOption.md)> |  |  |

### Return type

**String**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_markdown_raw

> String render_markdown_raw(body)
Render raw markdown as HTML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Request body to render | [required] |

### Return type

**String**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_markup

> String render_markup(body)
Render a markup document as HTML

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**MarkupOption**](MarkupOption.md)> |  |  |

### Return type

**String**

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

