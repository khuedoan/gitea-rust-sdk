# ContentsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | Option<[**models::FileLinksResponse**](FileLinksResponse.md)> |  | [optional]
**content** | Option<**String**> | `content` is populated when `type` is `file`, otherwise null | [optional]
**download_url** | Option<**String**> |  | [optional]
**encoding** | Option<**String**> | `encoding` is populated when `type` is `file`, otherwise null | [optional]
**git_url** | Option<**String**> |  | [optional]
**html_url** | Option<**String**> |  | [optional]
**last_commit_sha** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**sha** | Option<**String**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**submodule_git_url** | Option<**String**> | `submodule_git_url` is populated when `type` is `submodule`, otherwise null | [optional]
**target** | Option<**String**> | `target` is populated when `type` is `symlink`, otherwise null | [optional]
**r#type** | Option<**String**> | `type` will be `file`, `dir`, `symlink`, or `submodule` | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


