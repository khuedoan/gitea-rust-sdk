# Gitea\UserApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_current_user_repo**](UserApi.md#create_current_user_repo) | **POST** /user/repos | Create a repository
[**get_user_settings**](UserApi.md#get_user_settings) | **GET** /user/settings | Get user settings
[**get_verification_token**](UserApi.md#get_verification_token) | **GET** /user/gpg_key_token | Get a Token to verify
[**update_user_settings**](UserApi.md#update_user_settings) | **PATCH** /user/settings | Update user settings
[**user_add_email**](UserApi.md#user_add_email) | **POST** /user/emails | Add email addresses
[**user_check_following**](UserApi.md#user_check_following) | **GET** /users/{username}/following/{target} | Check if one user is following another user
[**user_create_o_auth2_application**](UserApi.md#user_create_o_auth2_application) | **POST** /user/applications/oauth2 | creates a new OAuth2 application
[**user_create_token**](UserApi.md#user_create_token) | **POST** /users/{username}/tokens | Create an access token
[**user_current_check_following**](UserApi.md#user_current_check_following) | **GET** /user/following/{username} | Check whether a user is followed by the authenticated user
[**user_current_check_starring**](UserApi.md#user_current_check_starring) | **GET** /user/starred/{owner}/{repo} | Whether the authenticated is starring the repo
[**user_current_delete_follow**](UserApi.md#user_current_delete_follow) | **DELETE** /user/following/{username} | Unfollow a user
[**user_current_delete_gpg_key**](UserApi.md#user_current_delete_gpg_key) | **DELETE** /user/gpg_keys/{id} | Remove a GPG key
[**user_current_delete_key**](UserApi.md#user_current_delete_key) | **DELETE** /user/keys/{id} | Delete a public key
[**user_current_delete_star**](UserApi.md#user_current_delete_star) | **DELETE** /user/starred/{owner}/{repo} | Unstar the given repo
[**user_current_get_gpg_key**](UserApi.md#user_current_get_gpg_key) | **GET** /user/gpg_keys/{id} | Get a GPG key
[**user_current_get_key**](UserApi.md#user_current_get_key) | **GET** /user/keys/{id} | Get a public key
[**user_current_list_followers**](UserApi.md#user_current_list_followers) | **GET** /user/followers | List the authenticated user's followers
[**user_current_list_following**](UserApi.md#user_current_list_following) | **GET** /user/following | List the users that the authenticated user is following
[**user_current_list_gpg_keys**](UserApi.md#user_current_list_gpg_keys) | **GET** /user/gpg_keys | List the authenticated user's GPG keys
[**user_current_list_keys**](UserApi.md#user_current_list_keys) | **GET** /user/keys | List the authenticated user's public keys
[**user_current_list_repos**](UserApi.md#user_current_list_repos) | **GET** /user/repos | List the repos that the authenticated user owns
[**user_current_list_starred**](UserApi.md#user_current_list_starred) | **GET** /user/starred | The repos that the authenticated user has starred
[**user_current_list_subscriptions**](UserApi.md#user_current_list_subscriptions) | **GET** /user/subscriptions | List repositories watched by the authenticated user
[**user_current_post_gpg_key**](UserApi.md#user_current_post_gpg_key) | **POST** /user/gpg_keys | Create a GPG key
[**user_current_post_key**](UserApi.md#user_current_post_key) | **POST** /user/keys | Create a public key
[**user_current_put_follow**](UserApi.md#user_current_put_follow) | **PUT** /user/following/{username} | Follow a user
[**user_current_put_star**](UserApi.md#user_current_put_star) | **PUT** /user/starred/{owner}/{repo} | Star the given repo
[**user_current_tracked_times**](UserApi.md#user_current_tracked_times) | **GET** /user/times | List the current user's tracked times
[**user_delete_access_token**](UserApi.md#user_delete_access_token) | **DELETE** /users/{username}/tokens/{token} | delete an access token
[**user_delete_email**](UserApi.md#user_delete_email) | **DELETE** /user/emails | Delete email addresses
[**user_delete_o_auth2_application**](UserApi.md#user_delete_o_auth2_application) | **DELETE** /user/applications/oauth2/{id} | delete an OAuth2 Application
[**user_get**](UserApi.md#user_get) | **GET** /users/{username} | Get a user
[**user_get_current**](UserApi.md#user_get_current) | **GET** /user | Get the authenticated user
[**user_get_heatmap_data**](UserApi.md#user_get_heatmap_data) | **GET** /users/{username}/heatmap | Get a user's heatmap
[**user_get_o_auth2_application**](UserApi.md#user_get_o_auth2_application) | **GET** /user/applications/oauth2/{id} | get an OAuth2 Application
[**user_get_oauth2_application**](UserApi.md#user_get_oauth2_application) | **GET** /user/applications/oauth2 | List the authenticated user's oauth2 applications
[**user_get_stop_watches**](UserApi.md#user_get_stop_watches) | **GET** /user/stopwatches | Get list of all existing stopwatches
[**user_get_tokens**](UserApi.md#user_get_tokens) | **GET** /users/{username}/tokens | List the authenticated user's access tokens
[**user_list_emails**](UserApi.md#user_list_emails) | **GET** /user/emails | List the authenticated user's email addresses
[**user_list_followers**](UserApi.md#user_list_followers) | **GET** /users/{username}/followers | List the given user's followers
[**user_list_following**](UserApi.md#user_list_following) | **GET** /users/{username}/following | List the users that the given user is following
[**user_list_gpg_keys**](UserApi.md#user_list_gpg_keys) | **GET** /users/{username}/gpg_keys | List the given user's GPG keys
[**user_list_keys**](UserApi.md#user_list_keys) | **GET** /users/{username}/keys | List the given user's public keys
[**user_list_repos**](UserApi.md#user_list_repos) | **GET** /users/{username}/repos | List the repos owned by the given user
[**user_list_starred**](UserApi.md#user_list_starred) | **GET** /users/{username}/starred | The repos that the given user has starred
[**user_list_subscriptions**](UserApi.md#user_list_subscriptions) | **GET** /users/{username}/subscriptions | List the repositories watched by a user
[**user_list_teams**](UserApi.md#user_list_teams) | **GET** /user/teams | List all the teams a user belongs to
[**user_search**](UserApi.md#user_search) | **GET** /users/search | Search for users
[**user_update_o_auth2_application**](UserApi.md#user_update_o_auth2_application) | **PATCH** /user/applications/oauth2/{id} | update an OAuth2 Application, this includes regenerating the client secret
[**user_verify_gpg_key**](UserApi.md#user_verify_gpg_key) | **POST** /user/gpg_key_verify | Verify a GPG key



## create_current_user_repo

> crate::models::Repository create_current_user_repo(body)
Create a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateRepoOption**](CreateRepoOption.md)> |  |  |

### Return type

[**crate::models::Repository**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings

> Vec<crate::models::UserSettings> get_user_settings()
Get user settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserSettings>**](UserSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_verification_token

> String get_verification_token()
Get a Token to verify

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_settings

> Vec<crate::models::UserSettings> update_user_settings(body)
Update user settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UserSettingsOptions**](UserSettingsOptions.md)> |  |  |

### Return type

[**Vec<crate::models::UserSettings>**](UserSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_add_email

> Vec<crate::models::Email> user_add_email(body)
Add email addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateEmailOption**](CreateEmailOption.md)> |  |  |

### Return type

[**Vec<crate::models::Email>**](Email.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_check_following

> user_check_following(username, target)
Check if one user is following another user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of following user | [required] |
**target** | **String** | username of followed user | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create_o_auth2_application

> crate::models::OAuth2Application user_create_o_auth2_application(body)
creates a new OAuth2 application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateOAuth2ApplicationOptions**](CreateOAuth2ApplicationOptions.md) |  | [required] |

### Return type

[**crate::models::OAuth2Application**](OAuth2Application.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_create_token

> crate::models::AccessToken user_create_token(username, user_create_token)
Create an access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**user_create_token** | Option<[**CreateAccessTokenOption**](CreateAccessTokenOption.md)> |  |  |

### Return type

[**crate::models::AccessToken**](AccessToken.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_check_following

> user_current_check_following(username)
Check whether a user is followed by the authenticated user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of followed user | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_check_starring

> user_current_check_starring(owner, repo)
Whether the authenticated is starring the repo

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


## user_current_delete_follow

> user_current_delete_follow(username)
Unfollow a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to unfollow | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_delete_gpg_key

> user_current_delete_gpg_key(id)
Remove a GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | id of key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_delete_key

> user_current_delete_key(id)
Delete a public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | id of key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_delete_star

> user_current_delete_star(owner, repo)
Unstar the given repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo to unstar | [required] |
**repo** | **String** | name of the repo to unstar | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_get_gpg_key

> crate::models::GpgKey user_current_get_gpg_key(id)
Get a GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | id of key to get | [required] |

### Return type

[**crate::models::GpgKey**](GPGKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_get_key

> crate::models::PublicKey user_current_get_key(id)
Get a public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | id of key to get | [required] |

### Return type

[**crate::models::PublicKey**](PublicKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_list_followers

> Vec<crate::models::User> user_current_list_followers(page, limit)
List the authenticated user's followers

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


## user_current_list_following

> Vec<crate::models::User> user_current_list_following(page, limit)
List the users that the authenticated user is following

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


## user_current_list_gpg_keys

> Vec<crate::models::GpgKey> user_current_list_gpg_keys(page, limit)
List the authenticated user's GPG keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::GpgKey>**](GPGKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_list_keys

> Vec<crate::models::PublicKey> user_current_list_keys(fingerprint, page, limit)
List the authenticated user's public keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fingerprint** | Option<**String**> | fingerprint of the key |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::PublicKey>**](PublicKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_list_repos

> Vec<crate::models::Repository> user_current_list_repos(page, limit)
List the repos that the authenticated user owns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_list_starred

> Vec<crate::models::Repository> user_current_list_starred(page, limit)
The repos that the authenticated user has starred

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_list_subscriptions

> Vec<crate::models::Repository> user_current_list_subscriptions(page, limit)
List repositories watched by the authenticated user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_post_gpg_key

> crate::models::GpgKey user_current_post_gpg_key(form)
Create a GPG key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form** | Option<[**CreateGpgKeyOption**](CreateGpgKeyOption.md)> |  |  |

### Return type

[**crate::models::GpgKey**](GPGKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_post_key

> crate::models::PublicKey user_current_post_key(body)
Create a public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateKeyOption**](CreateKeyOption.md)> |  |  |

### Return type

[**crate::models::PublicKey**](PublicKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_put_follow

> user_current_put_follow(username)
Follow a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to follow | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_put_star

> user_current_put_star(owner, repo)
Star the given repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo to star | [required] |
**repo** | **String** | name of the repo to star | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_current_tracked_times

> Vec<crate::models::TrackedTime> user_current_tracked_times(since, before)
List the current user's tracked times

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> | Only show times updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show times updated before the given time. This is a timestamp in RFC 3339 format |  |

### Return type

[**Vec<crate::models::TrackedTime>**](TrackedTime.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_access_token

> user_delete_access_token(username, token)
delete an access token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**token** | **String** | token to be deleted, identified by ID and if not available by name | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_email

> user_delete_email(body)
Delete email addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**DeleteEmailOption**](DeleteEmailOption.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_delete_o_auth2_application

> user_delete_o_auth2_application(id)
delete an OAuth2 Application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | token to be deleted | [required] |

### Return type

 (empty response body)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> crate::models::User user_get(username)
Get a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to get | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_current

> crate::models::User user_get_current()
Get the authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](User.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_heatmap_data

> Vec<crate::models::UserHeatmapData> user_get_heatmap_data(username)
Get a user's heatmap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user to get | [required] |

### Return type

[**Vec<crate::models::UserHeatmapData>**](UserHeatmapData.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_o_auth2_application

> crate::models::OAuth2Application user_get_o_auth2_application(id)
get an OAuth2 Application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | Application ID to be found | [required] |

### Return type

[**crate::models::OAuth2Application**](OAuth2Application.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_oauth2_application

> Vec<crate::models::OAuth2Application> user_get_oauth2_application(page, limit)
List the authenticated user's oauth2 applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::OAuth2Application>**](OAuth2Application.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_stop_watches

> Vec<crate::models::StopWatch> user_get_stop_watches(page, limit)
Get list of all existing stopwatches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::StopWatch>**](StopWatch.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get_tokens

> Vec<crate::models::AccessToken> user_get_tokens(username, page, limit)
List the authenticated user's access tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::AccessToken>**](AccessToken.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_emails

> Vec<crate::models::Email> user_list_emails()
List the authenticated user's email addresses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Email>**](Email.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_followers

> Vec<crate::models::User> user_list_followers(username, page, limit)
List the given user's followers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
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


## user_list_following

> Vec<crate::models::User> user_list_following(username, page, limit)
List the users that the given user is following

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
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


## user_list_gpg_keys

> Vec<crate::models::GpgKey> user_list_gpg_keys(username, page, limit)
List the given user's GPG keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::GpgKey>**](GPGKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_keys

> Vec<crate::models::PublicKey> user_list_keys(username, fingerprint, page, limit)
List the given user's public keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**fingerprint** | Option<**String**> | fingerprint of the key |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::PublicKey>**](PublicKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_repos

> Vec<crate::models::Repository> user_list_repos(username, page, limit)
List the repos owned by the given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_starred

> Vec<crate::models::Repository> user_list_starred(username, page, limit)
The repos that the given user has starred

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of user | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_subscriptions

> Vec<crate::models::Repository> user_list_subscriptions(username, page, limit)
List the repositories watched by a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | username of the user | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Repository>**](Repository.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_list_teams

> Vec<crate::models::Team> user_list_teams(page, limit)
List all the teams a user belongs to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_search

> crate::models::UserSearch200Response user_search(q, uid, page, limit)
Search for users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | keyword |  |
**uid** | Option<**i64**> | ID of the user to search for |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**crate::models::UserSearch200Response**](userSearch_200_response.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_update_o_auth2_application

> crate::models::OAuth2Application user_update_o_auth2_application(id, body)
update an OAuth2 Application, this includes regenerating the client secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | application to be updated | [required] |
**body** | [**CreateOAuth2ApplicationOptions**](CreateOAuth2ApplicationOptions.md) |  | [required] |

### Return type

[**crate::models::OAuth2Application**](OAuth2Application.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_verify_gpg_key

> crate::models::GpgKey user_verify_gpg_key()
Verify a GPG key

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GpgKey**](GPGKey.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

