use crate::error::ContractError;
use crate::msg::{
    AddrBalance, AddrBalanceResponse, AddrTokenInfoResponse, ContractAddr, ExecuteMsg,
    InstantiateMsg, QueryMsg,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdResult, Uint128,
    WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:multicall-cw20";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BulkBalance { contracts } => to_binary(&query_bulk_balance(deps, contracts)?),
        QueryMsg::BulkTokenInfo { contracts } => {
            to_binary(&query_bulk_token_info(deps, contracts)?)
        }
        _ => to_binary(&pass_query()?),
    }
}

pub fn pass_query() -> StdResult<Response> {
    Ok(Response::default())
}

pub fn query_bulk_balance(
    deps: Deps,
    contracts: Vec<AddrBalance>,
) -> StdResult<Vec<AddrBalanceResponse>> {
    let mut res: Vec<AddrBalanceResponse> = vec![];
    for contract in contracts.iter() {
        let response: BalanceResponse =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: contract.contract_addr.to_string(),
                msg: to_binary(&Cw20QueryMsg::Balance {
                    address: contract.address.to_string(),
                })?,
            }))?;

        res.push(AddrBalanceResponse {
            contract_addr: contract.contract_addr.to_string(),
            address: contract.address.to_string(),
            response,
        })
    }
    Ok(res)
}

pub fn query_bulk_token_info(
    deps: Deps,
    contracts: Vec<ContractAddr>,
) -> StdResult<Vec<AddrTokenInfoResponse>> {
    let mut res: Vec<AddrTokenInfoResponse> = vec![];
    for contract in contracts.iter() {
        let token_info_res: TokenInfoResponse =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: contract.contract_addr.to_string(),
                msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
            }))?;
        res.push(AddrTokenInfoResponse {
            contract_addr: contract.contract_addr.to_string(),
            response: token_info_res,
        })
    }
    Ok(res)
}
