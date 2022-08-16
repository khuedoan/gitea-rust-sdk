# EditRepoOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_manual_merge** | Option<**bool**> | either `true` to allow mark pr as merged manually, or `false` to prevent it. `has_pull_requests` must be `true`. | [optional]
**allow_merge_commits** | Option<**bool**> | either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. `has_pull_requests` must be `true`. | [optional]
**allow_rebase** | Option<**bool**> | either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. `has_pull_requests` must be `true`. | [optional]
**allow_rebase_explicit** | Option<**bool**> | either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits. `has_pull_requests` must be `true`. | [optional]
**allow_rebase_update** | Option<**bool**> | either `true` to allow updating pull request branch by rebase, or `false` to prevent it. `has_pull_requests` must be `true`. | [optional]
**allow_squash_merge** | Option<**bool**> | either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. `has_pull_requests` must be `true`. | [optional]
**archived** | Option<**bool**> | set to `true` to archive this repository. | [optional]
**autodetect_manual_merge** | Option<**bool**> | either `true` to enable AutodetectManualMerge, or `false` to prevent it. `has_pull_requests` must be `true`, Note: In some special cases, misjudgments can occur. | [optional]
**default_branch** | Option<**String**> | sets the default branch for this repository. | [optional]
**default_delete_branch_after_merge** | Option<**bool**> | set to `true` to delete pr branch after merge by default | [optional]
**default_merge_style** | Option<**String**> | set to a merge style to be used by this repository: \"merge\", \"rebase\", \"rebase-merge\", or \"squash\". `has_pull_requests` must be `true`. | [optional]
**description** | Option<**String**> | a short description of the repository. | [optional]
**enable_prune** | Option<**bool**> | enable prune - remove obsolete remote-tracking references | [optional]
**external_tracker** | Option<[**crate::models::ExternalTracker**](ExternalTracker.md)> |  | [optional]
**external_wiki** | Option<[**crate::models::ExternalWiki**](ExternalWiki.md)> |  | [optional]
**has_issues** | Option<**bool**> | either `true` to enable issues for this repository or `false` to disable them. | [optional]
**has_projects** | Option<**bool**> | either `true` to enable project unit, or `false` to disable them. | [optional]
**has_pull_requests** | Option<**bool**> | either `true` to allow pull requests, or `false` to prevent pull request. | [optional]
**has_wiki** | Option<**bool**> | either `true` to enable the wiki for this repository or `false` to disable it. | [optional]
**ignore_whitespace_conflicts** | Option<**bool**> | either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace. `has_pull_requests` must be `true`. | [optional]
**internal_tracker** | Option<[**crate::models::InternalTracker**](InternalTracker.md)> |  | [optional]
**mirror_interval** | Option<**String**> | set to a string like `8h30m0s` to set the mirror interval time | [optional]
**name** | Option<**String**> | name of the repository | [optional]
**private** | Option<**bool**> | either `true` to make the repository private or `false` to make it public. Note: you will get a 422 error if the organization restricts changing repository visibility to organization owners and a non-owner tries to change the value of private. | [optional]
**template** | Option<**bool**> | either `true` to make this repository a template or `false` to make it a normal repository | [optional]
**website** | Option<**String**> | a URL with more information about the repository. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


