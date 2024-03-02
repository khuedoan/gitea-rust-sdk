# Gitea\IssueApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**issue_add_label**](IssueApi.md#issue_add_label) | **POST** /repos/{owner}/{repo}/issues/{index}/labels | Add a label to an issue
[**issue_add_subscription**](IssueApi.md#issue_add_subscription) | **PUT** /repos/{owner}/{repo}/issues/{index}/subscriptions/{user} | Subscribe user to issue
[**issue_add_time**](IssueApi.md#issue_add_time) | **POST** /repos/{owner}/{repo}/issues/{index}/times | Add tracked time to a issue
[**issue_check_subscription**](IssueApi.md#issue_check_subscription) | **GET** /repos/{owner}/{repo}/issues/{index}/subscriptions/check | Check if user is subscribed to an issue
[**issue_clear_labels**](IssueApi.md#issue_clear_labels) | **DELETE** /repos/{owner}/{repo}/issues/{index}/labels | Remove all labels from an issue
[**issue_create_comment**](IssueApi.md#issue_create_comment) | **POST** /repos/{owner}/{repo}/issues/{index}/comments | Add a comment to an issue
[**issue_create_issue**](IssueApi.md#issue_create_issue) | **POST** /repos/{owner}/{repo}/issues | Create an issue. If using deadline only the date will be taken into account, and time of day ignored.
[**issue_create_issue_attachment**](IssueApi.md#issue_create_issue_attachment) | **POST** /repos/{owner}/{repo}/issues/{index}/assets | Create an issue attachment
[**issue_create_issue_blocking**](IssueApi.md#issue_create_issue_blocking) | **POST** /repos/{owner}/{repo}/issues/{index}/blocks | Block the issue given in the body by the issue in path
[**issue_create_issue_comment_attachment**](IssueApi.md#issue_create_issue_comment_attachment) | **POST** /repos/{owner}/{repo}/issues/comments/{id}/assets | Create a comment attachment
[**issue_create_issue_dependencies**](IssueApi.md#issue_create_issue_dependencies) | **POST** /repos/{owner}/{repo}/issues/{index}/dependencies | Make the issue in the url depend on the issue in the form.
[**issue_create_label**](IssueApi.md#issue_create_label) | **POST** /repos/{owner}/{repo}/labels | Create a label
[**issue_create_milestone**](IssueApi.md#issue_create_milestone) | **POST** /repos/{owner}/{repo}/milestones | Create a milestone
[**issue_delete**](IssueApi.md#issue_delete) | **DELETE** /repos/{owner}/{repo}/issues/{index} | Delete an issue
[**issue_delete_comment**](IssueApi.md#issue_delete_comment) | **DELETE** /repos/{owner}/{repo}/issues/comments/{id} | Delete a comment
[**issue_delete_comment_deprecated**](IssueApi.md#issue_delete_comment_deprecated) | **DELETE** /repos/{owner}/{repo}/issues/{index}/comments/{id} | Delete a comment
[**issue_delete_comment_reaction**](IssueApi.md#issue_delete_comment_reaction) | **DELETE** /repos/{owner}/{repo}/issues/comments/{id}/reactions | Remove a reaction from a comment of an issue
[**issue_delete_issue_attachment**](IssueApi.md#issue_delete_issue_attachment) | **DELETE** /repos/{owner}/{repo}/issues/{index}/assets/{attachment_id} | Delete an issue attachment
[**issue_delete_issue_comment_attachment**](IssueApi.md#issue_delete_issue_comment_attachment) | **DELETE** /repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id} | Delete a comment attachment
[**issue_delete_issue_reaction**](IssueApi.md#issue_delete_issue_reaction) | **DELETE** /repos/{owner}/{repo}/issues/{index}/reactions | Remove a reaction from an issue
[**issue_delete_label**](IssueApi.md#issue_delete_label) | **DELETE** /repos/{owner}/{repo}/labels/{id} | Delete a label
[**issue_delete_milestone**](IssueApi.md#issue_delete_milestone) | **DELETE** /repos/{owner}/{repo}/milestones/{id} | Delete a milestone
[**issue_delete_stop_watch**](IssueApi.md#issue_delete_stop_watch) | **DELETE** /repos/{owner}/{repo}/issues/{index}/stopwatch/delete | Delete an issue's existing stopwatch.
[**issue_delete_subscription**](IssueApi.md#issue_delete_subscription) | **DELETE** /repos/{owner}/{repo}/issues/{index}/subscriptions/{user} | Unsubscribe user from issue
[**issue_delete_time**](IssueApi.md#issue_delete_time) | **DELETE** /repos/{owner}/{repo}/issues/{index}/times/{id} | Delete specific tracked time
[**issue_edit_comment**](IssueApi.md#issue_edit_comment) | **PATCH** /repos/{owner}/{repo}/issues/comments/{id} | Edit a comment
[**issue_edit_comment_deprecated**](IssueApi.md#issue_edit_comment_deprecated) | **PATCH** /repos/{owner}/{repo}/issues/{index}/comments/{id} | Edit a comment
[**issue_edit_issue**](IssueApi.md#issue_edit_issue) | **PATCH** /repos/{owner}/{repo}/issues/{index} | Edit an issue. If using deadline only the date will be taken into account, and time of day ignored.
[**issue_edit_issue_attachment**](IssueApi.md#issue_edit_issue_attachment) | **PATCH** /repos/{owner}/{repo}/issues/{index}/assets/{attachment_id} | Edit an issue attachment
[**issue_edit_issue_comment_attachment**](IssueApi.md#issue_edit_issue_comment_attachment) | **PATCH** /repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id} | Edit a comment attachment
[**issue_edit_issue_deadline**](IssueApi.md#issue_edit_issue_deadline) | **POST** /repos/{owner}/{repo}/issues/{index}/deadline | Set an issue deadline. If set to null, the deadline is deleted. If using deadline only the date will be taken into account, and time of day ignored.
[**issue_edit_label**](IssueApi.md#issue_edit_label) | **PATCH** /repos/{owner}/{repo}/labels/{id} | Update a label
[**issue_edit_milestone**](IssueApi.md#issue_edit_milestone) | **PATCH** /repos/{owner}/{repo}/milestones/{id} | Update a milestone
[**issue_get_comment**](IssueApi.md#issue_get_comment) | **GET** /repos/{owner}/{repo}/issues/comments/{id} | Get a comment
[**issue_get_comment_reactions**](IssueApi.md#issue_get_comment_reactions) | **GET** /repos/{owner}/{repo}/issues/comments/{id}/reactions | Get a list of reactions from a comment of an issue
[**issue_get_comments**](IssueApi.md#issue_get_comments) | **GET** /repos/{owner}/{repo}/issues/{index}/comments | List all comments on an issue
[**issue_get_comments_and_timeline**](IssueApi.md#issue_get_comments_and_timeline) | **GET** /repos/{owner}/{repo}/issues/{index}/timeline | List all comments and events on an issue
[**issue_get_issue**](IssueApi.md#issue_get_issue) | **GET** /repos/{owner}/{repo}/issues/{index} | Get an issue
[**issue_get_issue_attachment**](IssueApi.md#issue_get_issue_attachment) | **GET** /repos/{owner}/{repo}/issues/{index}/assets/{attachment_id} | Get an issue attachment
[**issue_get_issue_comment_attachment**](IssueApi.md#issue_get_issue_comment_attachment) | **GET** /repos/{owner}/{repo}/issues/comments/{id}/assets/{attachment_id} | Get a comment attachment
[**issue_get_issue_reactions**](IssueApi.md#issue_get_issue_reactions) | **GET** /repos/{owner}/{repo}/issues/{index}/reactions | Get a list reactions of an issue
[**issue_get_label**](IssueApi.md#issue_get_label) | **GET** /repos/{owner}/{repo}/labels/{id} | Get a single label
[**issue_get_labels**](IssueApi.md#issue_get_labels) | **GET** /repos/{owner}/{repo}/issues/{index}/labels | Get an issue's labels
[**issue_get_milestone**](IssueApi.md#issue_get_milestone) | **GET** /repos/{owner}/{repo}/milestones/{id} | Get a milestone
[**issue_get_milestones_list**](IssueApi.md#issue_get_milestones_list) | **GET** /repos/{owner}/{repo}/milestones | Get all of a repository's opened milestones
[**issue_get_repo_comments**](IssueApi.md#issue_get_repo_comments) | **GET** /repos/{owner}/{repo}/issues/comments | List all comments in a repository
[**issue_list_blocks**](IssueApi.md#issue_list_blocks) | **GET** /repos/{owner}/{repo}/issues/{index}/blocks | List issues that are blocked by this issue
[**issue_list_issue_attachments**](IssueApi.md#issue_list_issue_attachments) | **GET** /repos/{owner}/{repo}/issues/{index}/assets | List issue's attachments
[**issue_list_issue_comment_attachments**](IssueApi.md#issue_list_issue_comment_attachments) | **GET** /repos/{owner}/{repo}/issues/comments/{id}/assets | List comment's attachments
[**issue_list_issue_dependencies**](IssueApi.md#issue_list_issue_dependencies) | **GET** /repos/{owner}/{repo}/issues/{index}/dependencies | List an issue's dependencies, i.e all issues that block this issue.
[**issue_list_issues**](IssueApi.md#issue_list_issues) | **GET** /repos/{owner}/{repo}/issues | List a repository's issues
[**issue_list_labels**](IssueApi.md#issue_list_labels) | **GET** /repos/{owner}/{repo}/labels | Get all of a repository's labels
[**issue_post_comment_reaction**](IssueApi.md#issue_post_comment_reaction) | **POST** /repos/{owner}/{repo}/issues/comments/{id}/reactions | Add a reaction to a comment of an issue
[**issue_post_issue_reaction**](IssueApi.md#issue_post_issue_reaction) | **POST** /repos/{owner}/{repo}/issues/{index}/reactions | Add a reaction to an issue
[**issue_remove_issue_blocking**](IssueApi.md#issue_remove_issue_blocking) | **DELETE** /repos/{owner}/{repo}/issues/{index}/blocks | Unblock the issue given in the body by the issue in path
[**issue_remove_issue_dependencies**](IssueApi.md#issue_remove_issue_dependencies) | **DELETE** /repos/{owner}/{repo}/issues/{index}/dependencies | Remove an issue dependency
[**issue_remove_label**](IssueApi.md#issue_remove_label) | **DELETE** /repos/{owner}/{repo}/issues/{index}/labels/{id} | Remove a label from an issue
[**issue_replace_labels**](IssueApi.md#issue_replace_labels) | **PUT** /repos/{owner}/{repo}/issues/{index}/labels | Replace an issue's labels
[**issue_reset_time**](IssueApi.md#issue_reset_time) | **DELETE** /repos/{owner}/{repo}/issues/{index}/times | Reset a tracked time of an issue
[**issue_search_issues**](IssueApi.md#issue_search_issues) | **GET** /repos/issues/search | Search for issues across the repositories that the user has access to
[**issue_start_stop_watch**](IssueApi.md#issue_start_stop_watch) | **POST** /repos/{owner}/{repo}/issues/{index}/stopwatch/start | Start stopwatch on an issue.
[**issue_stop_stop_watch**](IssueApi.md#issue_stop_stop_watch) | **POST** /repos/{owner}/{repo}/issues/{index}/stopwatch/stop | Stop an issue's existing stopwatch.
[**issue_subscriptions**](IssueApi.md#issue_subscriptions) | **GET** /repos/{owner}/{repo}/issues/{index}/subscriptions | Get users who subscribed on an issue.
[**issue_tracked_times**](IssueApi.md#issue_tracked_times) | **GET** /repos/{owner}/{repo}/issues/{index}/times | List an issue's tracked times
[**move_issue_pin**](IssueApi.md#move_issue_pin) | **PATCH** /repos/{owner}/{repo}/issues/{index}/pin/{position} | Moves the Pin to the given Position
[**pin_issue**](IssueApi.md#pin_issue) | **POST** /repos/{owner}/{repo}/issues/{index}/pin | Pin an Issue
[**unpin_issue**](IssueApi.md#unpin_issue) | **DELETE** /repos/{owner}/{repo}/issues/{index}/pin | Unpin an Issue



## issue_add_label

> Vec<models::Label> issue_add_label(owner, repo, index, body)
Add a label to an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**body** | Option<[**IssueLabelsOption**](IssueLabelsOption.md)> |  |  |

### Return type

[**Vec<models::Label>**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_add_subscription

> issue_add_subscription(owner, repo, index, user)
Subscribe user to issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**user** | **String** | user to subscribe | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_add_time

> models::TrackedTime issue_add_time(owner, repo, index, body)
Add tracked time to a issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**body** | Option<[**AddTimeOption**](AddTimeOption.md)> |  |  |

### Return type

[**models::TrackedTime**](TrackedTime.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_check_subscription

> models::WatchInfo issue_check_subscription(owner, repo, index)
Check if user is subscribed to an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |

### Return type

[**models::WatchInfo**](WatchInfo.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_clear_labels

> issue_clear_labels(owner, repo, index)
Remove all labels from an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_comment

> models::Comment issue_create_comment(owner, repo, index, body)
Add a comment to an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**body** | Option<[**CreateIssueCommentOption**](CreateIssueCommentOption.md)> |  |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_issue

> models::Issue issue_create_issue(owner, repo, body)
Create an issue. If using deadline only the date will be taken into account, and time of day ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**body** | Option<[**CreateIssueOption**](CreateIssueOption.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_issue_attachment

> models::Attachment issue_create_issue_attachment(owner, repo, index, attachment, name)
Create an issue attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**attachment** | **std::path::PathBuf** | attachment to upload | [required] |
**name** | Option<**String**> | name of the attachment |  |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_issue_blocking

> models::Issue issue_create_issue_blocking(owner, repo, index, body)
Block the issue given in the body by the issue in path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**body** | Option<[**IssueMeta**](IssueMeta.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_issue_comment_attachment

> models::Attachment issue_create_issue_comment_attachment(owner, repo, id, attachment, name)
Create a comment attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |
**attachment** | **std::path::PathBuf** | attachment to upload | [required] |
**name** | Option<**String**> | name of the attachment |  |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_issue_dependencies

> models::Issue issue_create_issue_dependencies(owner, repo, index, body)
Make the issue in the url depend on the issue in the form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**body** | Option<[**IssueMeta**](IssueMeta.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_label

> models::Label issue_create_label(owner, repo, body)
Create a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**body** | Option<[**CreateLabelOption**](CreateLabelOption.md)> |  |  |

### Return type

[**models::Label**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_create_milestone

> models::Milestone issue_create_milestone(owner, repo, body)
Create a milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**body** | Option<[**CreateMilestoneOption**](CreateMilestoneOption.md)> |  |  |

### Return type

[**models::Milestone**](Milestone.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete

> issue_delete(owner, repo, index)
Delete an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of issue to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_comment

> issue_delete_comment(owner, repo, id)
Delete a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of comment to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_comment_deprecated

> issue_delete_comment_deprecated(owner, repo, index, id)
Delete a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i32** | this parameter is ignored | [required] |
**id** | **i64** | id of comment to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_comment_reaction

> issue_delete_comment_reaction(owner, repo, id, content)
Remove a reaction from a comment of an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment to edit | [required] |
**content** | Option<[**EditReactionOption**](EditReactionOption.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_issue_attachment

> issue_delete_issue_attachment(owner, repo, index, attachment_id)
Delete an issue attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**attachment_id** | **i64** | id of the attachment to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_issue_comment_attachment

> issue_delete_issue_comment_attachment(owner, repo, id, attachment_id)
Delete a comment attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |
**attachment_id** | **i64** | id of the attachment to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_issue_reaction

> issue_delete_issue_reaction(owner, repo, index, content)
Remove a reaction from an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**content** | Option<[**EditReactionOption**](EditReactionOption.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_label

> issue_delete_label(owner, repo, id)
Delete a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the label to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_milestone

> issue_delete_milestone(owner, repo, id)
Delete a milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **String** | the milestone to delete, identified by ID and if not available by name | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_stop_watch

> issue_delete_stop_watch(owner, repo, index)
Delete an issue's existing stopwatch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to stop the stopwatch on | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_subscription

> issue_delete_subscription(owner, repo, index, user)
Unsubscribe user from issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**user** | **String** | user witch unsubscribe | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_delete_time

> issue_delete_time(owner, repo, index, id)
Delete specific tracked time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**id** | **i64** | id of time to delete | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_comment

> models::Comment issue_edit_comment(owner, repo, id, body)
Edit a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment to edit | [required] |
**body** | Option<[**EditIssueCommentOption**](EditIssueCommentOption.md)> |  |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_comment_deprecated

> models::Comment issue_edit_comment_deprecated(owner, repo, index, id, body)
Edit a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i32** | this parameter is ignored | [required] |
**id** | **i64** | id of the comment to edit | [required] |
**body** | Option<[**EditIssueCommentOption**](EditIssueCommentOption.md)> |  |  |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_issue

> models::Issue issue_edit_issue(owner, repo, index, body)
Edit an issue. If using deadline only the date will be taken into account, and time of day ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to edit | [required] |
**body** | Option<[**EditIssueOption**](EditIssueOption.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_issue_attachment

> models::Attachment issue_edit_issue_attachment(owner, repo, index, attachment_id, body)
Edit an issue attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**attachment_id** | **i64** | id of the attachment to edit | [required] |
**body** | Option<[**EditAttachmentOptions**](EditAttachmentOptions.md)> |  |  |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_issue_comment_attachment

> models::Attachment issue_edit_issue_comment_attachment(owner, repo, id, attachment_id, body)
Edit a comment attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |
**attachment_id** | **i64** | id of the attachment to edit | [required] |
**body** | Option<[**EditAttachmentOptions**](EditAttachmentOptions.md)> |  |  |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_issue_deadline

> models::IssueDeadline issue_edit_issue_deadline(owner, repo, index, body)
Set an issue deadline. If set to null, the deadline is deleted. If using deadline only the date will be taken into account, and time of day ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to create or update a deadline on | [required] |
**body** | Option<[**EditDeadlineOption**](EditDeadlineOption.md)> |  |  |

### Return type

[**models::IssueDeadline**](IssueDeadline.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_label

> models::Label issue_edit_label(owner, repo, id, body)
Update a label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the label to edit | [required] |
**body** | Option<[**EditLabelOption**](EditLabelOption.md)> |  |  |

### Return type

[**models::Label**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_edit_milestone

> models::Milestone issue_edit_milestone(owner, repo, id, body)
Update a milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **String** | the milestone to edit, identified by ID and if not available by name | [required] |
**body** | Option<[**EditMilestoneOption**](EditMilestoneOption.md)> |  |  |

### Return type

[**models::Milestone**](Milestone.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_comment

> models::Comment issue_get_comment(owner, repo, id)
Get a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |

### Return type

[**models::Comment**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_comment_reactions

> Vec<models::Reaction> issue_get_comment_reactions(owner, repo, id)
Get a list of reactions from a comment of an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment to edit | [required] |

### Return type

[**Vec<models::Reaction>**](Reaction.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_comments

> Vec<models::Comment> issue_get_comments(owner, repo, index, since, before)
List all comments on an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**since** | Option<**String**> | if provided, only comments updated since the specified time are returned. |  |
**before** | Option<**String**> | if provided, only comments updated before the provided time are returned. |  |

### Return type

[**Vec<models::Comment>**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_comments_and_timeline

> Vec<models::TimelineComment> issue_get_comments_and_timeline(owner, repo, index, since, page, limit, before)
List all comments and events on an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**since** | Option<**String**> | if provided, only comments updated since the specified time are returned. |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |
**before** | Option<**String**> | if provided, only comments updated before the provided time are returned. |  |

### Return type

[**Vec<models::TimelineComment>**](TimelineComment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_issue

> models::Issue issue_get_issue(owner, repo, index)
Get an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to get | [required] |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_issue_attachment

> models::Attachment issue_get_issue_attachment(owner, repo, index, attachment_id)
Get an issue attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**attachment_id** | **i64** | id of the attachment to get | [required] |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_issue_comment_attachment

> models::Attachment issue_get_issue_comment_attachment(owner, repo, id, attachment_id)
Get a comment attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |
**attachment_id** | **i64** | id of the attachment to get | [required] |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_issue_reactions

> Vec<models::Reaction> issue_get_issue_reactions(owner, repo, index, page, limit)
Get a list reactions of an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Reaction>**](Reaction.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_label

> models::Label issue_get_label(owner, repo, id)
Get a single label

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the label to get | [required] |

### Return type

[**models::Label**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_labels

> Vec<models::Label> issue_get_labels(owner, repo, index)
Get an issue's labels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |

### Return type

[**Vec<models::Label>**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_milestone

> models::Milestone issue_get_milestone(owner, repo, id)
Get a milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **String** | the milestone to get, identified by ID and if not available by name | [required] |

### Return type

[**models::Milestone**](Milestone.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_milestones_list

> Vec<models::Milestone> issue_get_milestones_list(owner, repo, state, name, page, limit)
Get all of a repository's opened milestones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**state** | Option<**String**> | Milestone state, Recognized values are open, closed and all. Defaults to \"open\" |  |
**name** | Option<**String**> | filter by milestone name |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Milestone>**](Milestone.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_get_repo_comments

> Vec<models::Comment> issue_get_repo_comments(owner, repo, since, before, page, limit)
List all comments in a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**since** | Option<**String**> | if provided, only comments updated since the provided time are returned. |  |
**before** | Option<**String**> | if provided, only comments updated before the provided time are returned. |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Comment>**](Comment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_blocks

> Vec<models::Issue> issue_list_blocks(owner, repo, index, page, limit)
List issues that are blocked by this issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Issue>**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_issue_attachments

> Vec<models::Attachment> issue_list_issue_attachments(owner, repo, index)
List issue's attachments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |

### Return type

[**Vec<models::Attachment>**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_issue_comment_attachments

> Vec<models::Attachment> issue_list_issue_comment_attachments(owner, repo, id)
List comment's attachments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment | [required] |

### Return type

[**Vec<models::Attachment>**](Attachment.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_issue_dependencies

> Vec<models::Issue> issue_list_issue_dependencies(owner, repo, index, page, limit)
List an issue's dependencies, i.e all issues that block this issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Issue>**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_issues

> Vec<models::Issue> issue_list_issues(owner, repo, state, labels, q, r#type, milestones, since, before, created_by, assigned_by, mentioned_by, page, limit)
List a repository's issues

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**state** | Option<**String**> | whether issue is open or closed |  |
**labels** | Option<**String**> | comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded |  |
**q** | Option<**String**> | search string |  |
**r#type** | Option<**String**> | filter by type (issues / pulls) if set |  |
**milestones** | Option<**String**> | comma separated list of milestone names or ids. It uses names and fall back to ids. Fetch only issues that have any of this milestones. Non existent milestones are discarded |  |
**since** | Option<**String**> | Only show items updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show items updated before the given time. This is a timestamp in RFC 3339 format |  |
**created_by** | Option<**String**> | Only show items which were created by the the given user |  |
**assigned_by** | Option<**String**> | Only show items for which the given user is assigned |  |
**mentioned_by** | Option<**String**> | Only show items in which the given user was mentioned |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Issue>**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_list_labels

> Vec<models::Label> issue_list_labels(owner, repo, page, limit)
Get all of a repository's labels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Label>**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_post_comment_reaction

> models::Reaction issue_post_comment_reaction(owner, repo, id, content)
Add a reaction to a comment of an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**id** | **i64** | id of the comment to edit | [required] |
**content** | Option<[**EditReactionOption**](EditReactionOption.md)> |  |  |

### Return type

[**models::Reaction**](Reaction.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_post_issue_reaction

> models::Reaction issue_post_issue_reaction(owner, repo, index, content)
Add a reaction to an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**content** | Option<[**EditReactionOption**](EditReactionOption.md)> |  |  |

### Return type

[**models::Reaction**](Reaction.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_remove_issue_blocking

> models::Issue issue_remove_issue_blocking(owner, repo, index, body)
Unblock the issue given in the body by the issue in path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**body** | Option<[**IssueMeta**](IssueMeta.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_remove_issue_dependencies

> models::Issue issue_remove_issue_dependencies(owner, repo, index, body)
Remove an issue dependency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **String** | index of the issue | [required] |
**body** | Option<[**IssueMeta**](IssueMeta.md)> |  |  |

### Return type

[**models::Issue**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_remove_label

> issue_remove_label(owner, repo, index, id)
Remove a label from an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**id** | **i64** | id of the label to remove | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_replace_labels

> Vec<models::Label> issue_replace_labels(owner, repo, index, body)
Replace an issue's labels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**body** | Option<[**IssueLabelsOption**](IssueLabelsOption.md)> |  |  |

### Return type

[**Vec<models::Label>**](Label.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_reset_time

> issue_reset_time(owner, repo, index)
Reset a tracked time of an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to add tracked time to | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_search_issues

> Vec<models::Issue> issue_search_issues(state, labels, milestones, q, priority_repo_id, r#type, since, before, assigned, created, mentioned, review_requested, reviewed, owner, team, page, limit)
Search for issues across the repositories that the user has access to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | whether issue is open or closed |  |
**labels** | Option<**String**> | comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded |  |
**milestones** | Option<**String**> | comma separated list of milestone names. Fetch only issues that have any of this milestones. Non existent are discarded |  |
**q** | Option<**String**> | search string |  |
**priority_repo_id** | Option<**i64**> | repository to prioritize in the results |  |
**r#type** | Option<**String**> | filter by type (issues / pulls) if set |  |
**since** | Option<**String**> | Only show notifications updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show notifications updated before the given time. This is a timestamp in RFC 3339 format |  |
**assigned** | Option<**bool**> | filter (issues / pulls) assigned to you, default is false |  |
**created** | Option<**bool**> | filter (issues / pulls) created by you, default is false |  |
**mentioned** | Option<**bool**> | filter (issues / pulls) mentioning you, default is false |  |
**review_requested** | Option<**bool**> | filter pulls requesting your review, default is false |  |
**reviewed** | Option<**bool**> | filter pulls reviewed by you, default is false |  |
**owner** | Option<**String**> | filter by owner |  |
**team** | Option<**String**> | filter by team (requires organization owner parameter to be provided) |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::Issue>**](Issue.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_start_stop_watch

> issue_start_stop_watch(owner, repo, index)
Start stopwatch on an issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to create the stopwatch on | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_stop_stop_watch

> issue_stop_stop_watch(owner, repo, index)
Stop an issue's existing stopwatch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue to stop the stopwatch on | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_subscriptions

> Vec<models::User> issue_subscriptions(owner, repo, index, page, limit)
Get users who subscribed on an issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::User>**](User.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_tracked_times

> Vec<models::TrackedTime> issue_tracked_times(owner, repo, index, user, since, before, page, limit)
List an issue's tracked times

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of the issue | [required] |
**user** | Option<**String**> | optional filter by user (available for issue managers) |  |
**since** | Option<**String**> | Only show times updated after the given time. This is a timestamp in RFC 3339 format |  |
**before** | Option<**String**> | Only show times updated before the given time. This is a timestamp in RFC 3339 format |  |
**page** | Option<**i32**> | page number of results to return (1-based) |  |
**limit** | Option<**i32**> | page size of results |  |

### Return type

[**Vec<models::TrackedTime>**](TrackedTime.md)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_issue_pin

> move_issue_pin(owner, repo, index, position)
Moves the Pin to the given Position

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of issue | [required] |
**position** | **i64** | the new position | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_issue

> pin_issue(owner, repo, index)
Pin an Issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of issue to pin | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpin_issue

> unpin_issue(owner, repo, index)
Unpin an Issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | owner of the repo | [required] |
**repo** | **String** | name of the repo | [required] |
**index** | **i64** | index of issue to unpin | [required] |

### Return type

 (empty response body)

### Authorization

[TOTPHeader](../README.md#TOTPHeader), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [SudoHeader](../README.md#SudoHeader), [BasicAuth](../README.md#BasicAuth), [AccessToken](../README.md#AccessToken), [SudoParam](../README.md#SudoParam), [Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

