use cosmwasm_std::{Addr, Binary, Uint128};
use cw0::Expiration;
use cw20::{AllowanceInfo, BalanceResponse, TokenInfoResponse, AllowanceResponse, MinterResponse, MarketingInfoResponse, DownloadLogoResponse, AllAllowancesResponse, AllAccountsResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractAddr {
    pub contract_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddrBalance {
    pub contract_addr: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddrAllowance {
    pub contract_addr: String,
    pub owner: String,
    pub spender: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddrAllAllowance {
    pub contract_addr: String,
    pub owner: String,
    pub start_after: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddrAllAccounts {
    pub contract_addr: String,
    pub start_after: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    BulkBalance { contracts: Vec<AddrBalance> },
      /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    BulkTokenInfo { contracts: Vec<ContractAddr> },
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    BulkAllowance { contracts: Vec<AddrAllowance> },
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.
    BulkMinter { contracts: Vec<ContractAddr> },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse.
    BulkMarketingInfo { contracts: Vec<ContractAddr> },
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data stored for
    /// this contract.
    /// Return type: DownloadLogoResponse.
    BulkDownloadLogo { contracts: Vec<ContractAddr> },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    BulkAllAllowances { contracts: Vec<AddrAllAllowance> },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    BulkAllAccounts { contracts: Vec<AddrAllAccounts> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrBalanceResponse {
    pub contract_addr: String,
    pub address: String,
    pub response: BalanceResponse,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrTokenInfoResponse {
    pub contract_addr: String,
    pub response: TokenInfoResponse
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AddrAllowanceResponse {
    pub contract_addr: String,
    pub response: AllowanceResponse
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrMinterResponse {
    pub contract_addr: String,
    pub response: MinterResponse
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AddrMarketingInfoResponse {
    pub contract_addr: String,
    pub response: MarketingInfoResponse
}

/// When we download an embedded logo, we get this response type.
/// We expect a SPA to be able to accept this info and display it.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrDownloadLogoResponse {
    pub contract_addr: String,
    pub response: DownloadLogoResponse
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrAllowanceInfo {
    pub contract_addr: String,
    pub response: AllowanceInfo
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrAllAllowancesResponse {
    pub contract_addr: String,
    pub response: AllAllowancesResponse
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AddrAllAccountsResponse {
    pub contract_addr: String,
    pub response: AllAccountsResponse
}
