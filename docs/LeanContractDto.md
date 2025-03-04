# LeanContractDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the contract template | 
**name** | **String** | The name of the contract template | 
**description** | **String** | A short description of the contract template | 
**attributes** | Option<[**models::ContractAttributes**](ContractAttributes.md)> | The attributes related to this contract template. It will be displayed in the tokenization page | [optional]
**is_public** | **bool** | Is this a contract that is viewable by all fireblocks's users or is it visible only for this workspace | 
**can_deploy** | Option<**bool**> | True if the workspace allowed to deploy this contract, false otherwise | [optional]
**owner** | Option<**String**> | The workspace id of the owner of this contract template. If it's a private contract, only this workspace will be allowed to deploy it | [optional]
**vendor** | Option<[**models::VendorDto**](VendorDto.md)> | The details of the vendor of this contract template. Applicable only for public contract templates | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


