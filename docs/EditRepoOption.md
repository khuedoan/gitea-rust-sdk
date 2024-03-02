# EditRepoOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_fast_forward_only_merge** | Option<**bool**> | either `true` to allow fast-forward-only merging pull requests, or `false` to prevent fast-forward-only merging. | [optional]
**allow_manual_merge** | Option<**bool**> | either `true` to allow mark pr as merged manually, or `false` to prevent it. | [optional]
**allow_merge_commits** | Option<**bool**> | either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits. | [optional]
**allow_rebase** | Option<**bool**> | either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging. | [optional]
**allow_rebase_explicit** | Option<**bool**> | either `true` to allow rebase with explicit merge commits (--no-ff), or `false` to prevent rebase with explicit merge commits. | [optional]
**allow_rebase_update** | Option<**bool**> | either `true` to allow updating pull request branch by rebase, or `false` to prevent it. | [optional]
**allow_squash_merge** | Option<**bool**> | either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging. | [optional]
**archived** | Option<**bool**> | set to `true` to archive this repository. | [optional]
**autodetect_manual_merge** | Option<**bool**> | either `true` to enable AutodetectManualMerge, or `false` to prevent it. Note: In some special cases, misjudgments can occur. | [optional]
**default_allow_maintainer_edit** | Option<**bool**> | set to `true` to allow edits from maintainers by default | [optional]
**default_branch** | Option<**String**> | sets the default branch for this repository. | [optional]
**default_delete_branch_after_merge** | Option<**bool**> | set to `true` to delete pr branch after merge by default | [optional]
**default_merge_style** | Option<**String**> | set to a merge style to be used by this repository: \"merge\", \"rebase\", \"rebase-merge\", \"squash\", or \"fast-forward-only\". | [optional]
**description** | Option<**String**> | a short description of the repository. | [optional]
**enable_prune** | Option<**bool**> | enable prune - remove obsolete remote-tracking references | [optional]
**external_tracker** | Option<[**models::ExternalTracker**](ExternalTracker.md)> |  | [optional]
**external_wiki** | Option<[**models::ExternalWiki**](ExternalWiki.md)> |  | [optional]
**has_actions** | Option<**bool**> | either `true` to enable actions unit, or `false` to disable them. | [optional]
**has_issues** | Option<**bool**> | either `true` to enable issues for this repository or `false` to disable them. | [optional]
**has_packages** | Option<**bool**> | either `true` to enable packages unit, or `false` to disable them. | [optional]
**has_projects** | Option<**bool**> | either `true` to enable project unit, or `false` to disable them. | [optional]
**has_pull_requests** | Option<**bool**> | either `true` to allow pull requests, or `false` to prevent pull request. | [optional]
**has_releases** | Option<**bool**> | either `true` to enable releases unit, or `false` to disable them. | [optional]
**has_wiki** | Option<**bool**> | either `true` to enable the wiki for this repository or `false` to disable it. | [optional]
**ignore_whitespace_conflicts** | Option<**bool**> | either `true` to ignore whitespace for conflicts, or `false` to not ignore whitespace. | [optional]
**internal_tracker** | Option<[**models::InternalTracker**](InternalTracker.md)> |  | [optional]
**mirror_interval** | Option<**String**> | set to a string like `8h30m0s` to set the mirror interval time | [optional]
**name** | Option<**String**> | name of the repository | [optional]
**private** | Option<**bool**> | either `true` to make the repository private or `false` to make it public. Note: you will get a 422 error if the organization restricts changing repository visibility to organization owners and a non-owner tries to change the value of private. | [optional]
**template** | Option<**bool**> | either `true` to make this repository a template or `false` to make it a normal repository | [optional]
**website** | Option<**String**> | a URL with more information about the repository. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


