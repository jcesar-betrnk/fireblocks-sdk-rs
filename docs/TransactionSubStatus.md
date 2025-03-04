# TransactionSubStatus

## Enum Variants

| Name | Value |
|---- | -----|
| Variant3RdPartyProcessing | 3RD_PARTY_PROCESSING |
| Variant3RdPartyPendingServiceManualApproval | 3RD_PARTY_PENDING_SERVICE_MANUAL_APPROVAL |
| Pending3RdPartyManualApproval | PENDING_3RD_PARTY_MANUAL_APPROVAL |
| Variant3RdPartyConfirming | 3RD_PARTY_CONFIRMING |
| PendingBlockchainConfirmations | PENDING_BLOCKCHAIN_CONFIRMATIONS |
| Variant3RdPartyCompleted | 3RD_PARTY_COMPLETED |
| CompletedBut3RdPartyFailed | COMPLETED_BUT_3RD_PARTY_FAILED |
| CompletedBut3RdPartyRejected | COMPLETED_BUT_3RD_PARTY_REJECTED |
| Confirmed | CONFIRMED |
| BlockedByPolicy | BLOCKED_BY_POLICY |
| Variant3RdPartyCancelled | 3RD_PARTY_CANCELLED |
| Variant3RdPartyRejected | 3RD_PARTY_REJECTED |
| CancelledByUser | CANCELLED_BY_USER |
| CancelledByUserRequest | CANCELLED_BY_USER_REQUEST |
| RejectedByUser | REJECTED_BY_USER |
| AutoFreeze | AUTO_FREEZE |
| FrozenManually | FROZEN_MANUALLY |
| RejectedAmlScreening | REJECTED_AML_SCREENING |
| ActualFeeTooHigh | ACTUAL_FEE_TOO_HIGH |
| AddressWhitelistingSuspended | ADDRESS_WHITELISTING_SUSPENDED |
| AmountTooSmall | AMOUNT_TOO_SMALL |
| AuthorizationFailed | AUTHORIZATION_FAILED |
| AuthorizerNotFound | AUTHORIZER_NOT_FOUND |
| EnvUnsupportedAsset | ENV_UNSUPPORTED_ASSET |
| ErrorUnsupportedTransactionType | ERROR_UNSUPPORTED_TRANSACTION_TYPE |
| FailOnLowFee | FAIL_ON_LOW_FEE |
| GasLimitTooLow | GAS_LIMIT_TOO_LOW |
| GasPriceTooLowForRbf | GAS_PRICE_TOO_LOW_FOR_RBF |
| IncompleteUserSetup | INCOMPLETE_USER_SETUP |
| InsufficientFunds | INSUFFICIENT_FUNDS |
| InsufficientFundsForFee | INSUFFICIENT_FUNDS_FOR_FEE |
| IntegrationSuspended | INTEGRATION_SUSPENDED |
| InvalidAddress | INVALID_ADDRESS |
| InvalidContractCallData | INVALID_CONTRACT_CALL_DATA |
| InvalidFeeParams | INVALID_FEE_PARAMS |
| InvalidNonceForRbf | INVALID_NONCE_FOR_RBF |
| InvalidTagOrMemo | INVALID_TAG_OR_MEMO |
| InvalidUnmanagedWallet | INVALID_UNMANAGED_WALLET |
| MaxFeeExceeded | MAX_FEE_EXCEEDED |
| MissingTagOrMemo | MISSING_TAG_OR_MEMO |
| NeedMoreToCreateDestination | NEED_MORE_TO_CREATE_DESTINATION |
| NoMorePreprocessedIndexes | NO_MORE_PREPROCESSED_INDEXES |
| NonExistingAccountName | NON_EXISTING_ACCOUNT_NAME |
| RawMsgEmptyOrInvalid | RAW_MSG_EMPTY_OR_INVALID |
| RawMsgLenInvalid | RAW_MSG_LEN_INVALID |
| TooManyInputs | TOO_MANY_INPUTS |
| TxSizeExceededMax | TX_SIZE_EXCEEDED_MAX |
| UnauthorisedDevice | UNAUTHORISED_DEVICE |
| UnauthorisedUser | UNAUTHORISED_USER |
| UnallowedRawParamCombination | UNALLOWED_RAW_PARAM_COMBINATION |
| UnsupportedOperation | UNSUPPORTED_OPERATION |
| UnsupportedTransactionType | UNSUPPORTED_TRANSACTION_TYPE |
| ZeroBalanceInPermanentAddress | ZERO_BALANCE_IN_PERMANENT_ADDRESS |
| OutOfDateSigningKeys | OUT_OF_DATE_SIGNING_KEYS |
| ConnectivityError | CONNECTIVITY_ERROR |
| ErrorAsyncTxInFlight | ERROR_ASYNC_TX_IN_FLIGHT |
| InternalError | INTERNAL_ERROR |
| InvalidNonceTooHigh | INVALID_NONCE_TOO_HIGH |
| InvalidNonceTooLow | INVALID_NONCE_TOO_LOW |
| InvalidRoutingDestination | INVALID_ROUTING_DESTINATION |
| LockingNonceAccountTimeout | LOCKING_NONCE_ACCOUNT_TIMEOUT |
| NetworkRoutingMismatch | NETWORK_ROUTING_MISMATCH |
| NonceAllocationFailed | NONCE_ALLOCATION_FAILED |
| ResourceAlreadyExists | RESOURCE_ALREADY_EXISTS |
| SignerNotFound | SIGNER_NOT_FOUND |
| SigningError | SIGNING_ERROR |
| Timeout | TIMEOUT |
| TxOutdated | TX_OUTDATED |
| UnknownError | UNKNOWN_ERROR |
| VaultWalletNotReady | VAULT_WALLET_NOT_READY |
| UnsupportedMediaType | UNSUPPORTED_MEDIA_TYPE |
| AddressNotWhitelisted | ADDRESS_NOT_WHITELISTED |
| ApiKeyMismatch | API_KEY_MISMATCH |
| AssetNotEnabledOnDestination | ASSET_NOT_ENABLED_ON_DESTINATION |
| DestTypeNotSupported | DEST_TYPE_NOT_SUPPORTED |
| ExceededDecimalPrecision | EXCEEDED_DECIMAL_PRECISION |
| ExchangeConfigurationMismatch | EXCHANGE_CONFIGURATION_MISMATCH |
| ExchangeVersionIncompatible | EXCHANGE_VERSION_INCOMPATIBLE |
| InvalidExchangeAccount | INVALID_EXCHANGE_ACCOUNT |
| MethodNotAllowed | METHOD_NOT_ALLOWED |
| NonExistentAutoAccount | NON_EXISTENT_AUTO_ACCOUNT |
| OnPremiseConnectivityError | ON_PREMISE_CONNECTIVITY_ERROR |
| PeerAccountDoesNotExist | PEER_ACCOUNT_DOES_NOT_EXIST |
| ThirdPartyMissingAccount | THIRD_PARTY_MISSING_ACCOUNT |
| UnauthorisedIpWhitelisting | UNAUTHORISED_IP_WHITELISTING |
| UnauthorisedMissingCredentials | UNAUTHORISED_MISSING_CREDENTIALS |
| UnauthorisedMissingPermission | UNAUTHORISED_MISSING_PERMISSION |
| UnauthorisedOtpFailed | UNAUTHORISED_OTP_FAILED |
| WithdrawLimit | WITHDRAW_LIMIT |
| Variant3RdPartyFailed | 3RD_PARTY_FAILED |
| ApiCallLimit | API_CALL_LIMIT |
| ApiInvalidSignature | API_INVALID_SIGNATURE |
| CancelledExternally | CANCELLED_EXTERNALLY |
| FailedAmlScreening | FAILED_AML_SCREENING |
| InvalidFee | INVALID_FEE |
| InvalidThirdPartyResponse | INVALID_THIRD_PARTY_RESPONSE |
| ManualDepositAddressRequired | MANUAL_DEPOSIT_ADDRESS_REQUIRED |
| MissingDepositAddress | MISSING_DEPOSIT_ADDRESS |
| NoDepositAddress | NO_DEPOSIT_ADDRESS |
| SubAccountsNotSupported | SUB_ACCOUNTS_NOT_SUPPORTED |
| SpendCoinbaseTooEarly | SPEND_COINBASE_TOO_EARLY |
| ThirdPartyInternalError | THIRD_PARTY_INTERNAL_ERROR |
| TxIdNotAcceptedByThirdParty | TX_ID_NOT_ACCEPTED_BY_THIRD_PARTY |
| UnsupportedAsset | UNSUPPORTED_ASSET |
| DoubleSpending | DOUBLE_SPENDING |
| DroppedByBlockchain | DROPPED_BY_BLOCKCHAIN |
| InsufficientReservedFunding | INSUFFICIENT_RESERVED_FUNDING |
| InvalidSignature | INVALID_SIGNATURE |
| PartiallyFailed | PARTIALLY_FAILED |
| PowerupSuggestionFailure | POWERUP_SUGGESTION_FAILURE |
| ReachedMempoolLimitForAccount | REACHED_MEMPOOL_LIMIT_FOR_ACCOUNT |
| RejectedByBlockchain | REJECTED_BY_BLOCKCHAIN |
| SmartContractExecutionFailed | SMART_CONTRACT_EXECUTION_FAILED |
| TooLongMempoolChain | TOO_LONG_MEMPOOL_CHAIN |


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


