# Gitea\AdminApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_adopt_repository**](AdminApi.md#admin_adopt_repository) | **POST** /admin/unadopted/{owner}/{repo} | Adopt unadopted files as a repository
[**admin_create_org**](AdminApi.md#admin_create_org) | **POST** /admin/users/{username}/orgs | Create an organization
[**admin_create_public_key**](AdminApi.md#admin_create_public_key) | **POST** /admin/users/{username}/keys | Add a public key on behalf of a user
[**admin_create_repo**](AdminApi.md#admin_create_repo) | **POST** /admin/users/{username}/repos | Create a repository on behalf of a user
[**admin_create_user**](AdminApi.md#admin_create_user) | **POST** /admin/users | Create a user
[**admin_cron_list**](AdminApi.md#admin_cron_list) | **GET** /admin/cron | List cron tasks
[**admin_cron_run**](AdminApi.md#admin_cron_run) | **POST** /admin/cron/{task} | Run cron task
[**admin_delete_unadopted_repository**](AdminApi.md#admin_delete_unadopted_repository) | **DELETE** /admin/unadopted/{owner}/{repo} | Delete unadopted files
[**admin_delete_user**](AdminApi.md#admin_delete_user) | **DELETE** /admin/users/{username} | Delete a user
[**admin_delete_user_public_key**](AdminApi.md#admin_delete_user_public_key) | **DELETE** /admin/users/{username}/keys/{id} | Delete a user's public key
[**admin_edit_user**](AdminApi.md#admin_edit_user) | **PATCH** /admin/users/{username} | Edit an existing user
[**admin_get_all_orgs**](AdminApi.md#admin_get_all_orgs) | **GET** /admin/orgs | List all organizations
[**admin_get_all_users**](AdminApi.md#admin_get_all_users) | **GET** /admin/users | List all users
[**admin_unadopted_list**](AdminApi.md#admin_unadopted_list) | **GET** /admin/unadopted | List unadopted repositories



## admin_adopt_repository

> admin_adopt_repository(owner, repo)
Adopt unadopted files as a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_create_org

> crate::models::Organization admin_create_org(username, organization)
Create an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of the user that will own the created organization | [required] |
**organization** | [**CreateOrgOption**](CreateOrgOption.md) |  | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_create_public_key

> crate::models::PublicKey admin_create_public_key(username, key)
Add a public key on behalf of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of the user | [required] |
**key** | Option<[**CreateKeyOption**](CreateKeyOption.md)> |  |  |

### Return type

[**crate::models::PublicKey**](PublicKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_create_repo

> crate::models::Repository admin_create_repo(username, repository)
Create a repository on behalf of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of the user. This user will own the created repository | [required] |
**repository** | [**CreateRepoOption**](CreateRepoOption.md) |  | [required] |

### Return type

[**crate::models::Repository**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_create_user

> crate::models::User admin_create_user(body)
Create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateUserOption**](CreateUserOption.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_cron_list

> Vec<crate::models::Cron> admin_cron_list(page, limit)
List cron tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Cron>**](Cron.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_cron_run

> admin_cron_run(task)
Run cron task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task** | **String** | task to run | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_delete_unadopted_repository

> admin_delete_unadopted_repository(owner, repo)
Delete unadopted files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_delete_user

> admin_delete_user(username)
Delete a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to delete | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_delete_user_public_key

> admin_delete_user_public_key(username, id)
Delete a user's public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**id** | **i64** | id of the key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_edit_user

> crate::models::User admin_edit_user(username, body)
Edit an existing user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to edit | [required] |
**body** | Option<[**EditUserOption**](EditUserOption.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_get_all_orgs

> Vec<crate::models::Organization> admin_get_all_orgs(page, limit)
List all organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Organization>**](Organization.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_get_all_users

> Vec<crate::models::User> admin_get_all_users(page, limit)
List all users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_unadopted_list

> Vec<String> admin_unadopted_list(page, limit, pattern)
List unadopted repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |
**pattern** | Option<**String**> | pattern of repositories to search for |  |

### Return type

**Vec<String>**

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

