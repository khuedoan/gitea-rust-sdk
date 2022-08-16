# Gitea\PackageApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_package**](PackageApi.md#delete_package) | **DELETE** /packages/{owner}/{type}/{name}/{version} | Delete a package
[**get_package**](PackageApi.md#get_package) | **GET** /packages/{owner}/{type}/{name}/{version} | Gets a package
[**list_package_files**](PackageApi.md#list_package_files) | **GET** /packages/{owner}/{type}/{name}/{version}/files | Gets all files of a package
[**list_packages**](PackageApi.md#list_packages) | **GET** /packages/{owner} | Gets all packages of an owner



## delete_package

> delete_package(owner, _type, name, version)
Delete a package

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the package | [required] |
**_type** | **String** | type of the package | [required] |
**name** | **String** | name of the package | [required] |
**version** | **String** | version of the package | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_package

> crate::models::Package get_package(owner, _type, name, version)
Gets a package

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the package | [required] |
**_type** | **String** | type of the package | [required] |
**name** | **String** | name of the package | [required] |
**version** | **String** | version of the package | [required] |

### Return type

[**crate::models::Package**](Package.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_package_files

> Vec<crate::models::PackageFile> list_package_files(owner, _type, name, version)
Gets all files of a package

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the package | [required] |
**_type** | **String** | type of the package | [required] |
**name** | **String** | name of the package | [required] |
**version** | **String** | version of the package | [required] |

### Return type

[**Vec<crate::models::PackageFile>**](PackageFile.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_packages

> Vec<crate::models::Package> list_packages(owner, page, limit, _type, q)
Gets all packages of an owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the packages | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |
**_type** | Option<**String**> | package type filter |  |
**q** | Option<**String**> | name filter |  |

### Return type

[**Vec<crate::models::Package>**](Package.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

