# UpdateFileOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<[**crate::models::Identity**](Identity.md)> |  | [optional]
**branch** | Option<**String**> | branch (optional) to base this file from. if not given, the default branch is used | [optional]
**committer** | Option<[**crate::models::Identity**](Identity.md)> |  | [optional]
**content** | **String** | content must be base64 encoded | 
**dates** | Option<[**crate::models::CommitDateOptions**](CommitDateOptions.md)> |  | [optional]
**from_path** | Option<**String**> | from_path (optional) is the path of the original file which will be moved/renamed to the path in the URL | [optional]
**message** | Option<**String**> | message (optional) for the commit of this file. if not supplied, a default message will be used | [optional]
**new_branch** | Option<**String**> | new_branch (optional) will make a new branch from `branch` before creating the file | [optional]
**sha** | **String** | sha is the SHA for the file that already exists | 
**signoff** | Option<**bool**> | Add a Signed-off-by trailer by the committer at the end of the commit log message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


