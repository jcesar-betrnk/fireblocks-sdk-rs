# Delegation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the staking position | 
**vault_account_id** | **String** | The source vault account to stake from | 
**validator_name** | **String** | The destination validator address name | 
**provider_name** | **String** | The destination validator provider name | 
**chain_descriptor** | **String** | The protocol identifier (e.g. \"ETH\"/ \"SOL\") to use | 
**amount** | **String** | Amount of tokens to stake, measured in the staked asset unit. | 
**rewards_amount** | **String** | The amount staked in the position, measured in the staked asset unit. | 
**date_created** | **String** | When was the request made (ISO Date). | 
**date_updated** | **String** | When has the position last changed (ISO Date). | 
**status** | **String** | The current status. | 
**related_transactions** | [**Vec<models::RelatedTransaction>**](RelatedTransaction.md) | An array of transaction objects related to this position. Each object includes a 'txId' representing the transaction ID and a 'completed' boolean indicating if the transaction was completed. | 
**validator_address** | **String** | The destination address of the staking transaction. | 
**provider_id** | [**models::StakingProvider**](StakingProvider.md) |  | 
**available_actions** | **Vec<String>** | An array of available actions that can be performed. for example, actions like \"unstake\" or \"withdraw\". | 
**in_progress** | **bool** | Indicates whether there is an ongoing action for this position (true if ongoing, false if not). | 
**in_progress_tx_id** | Option<**String**> | The transaction ID of the ongoing request | [optional]
**blockchain_position_info** | [**models::SolanaBlockchainData**](SolanaBlockchainData.md) |  | 
**related_requests** | Option<[**Vec<models::RelatedRequest>**](RelatedRequest.md)> | An array of partial unstake requests for this position, relevant only for the Lido provider. Each object includes the status of the unstake request, a boolean indicating whether the action is in progress, the amount of tokens to unstake, and the transaction ID of the request. With Lido, a position may have multiple partial unstake requests in different states. This field is optional and not applicable for other providers. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


