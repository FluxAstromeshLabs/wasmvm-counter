use std::result;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult,
};
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Count {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(i32)]
    Value {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Counter {
    pub value: i32,
}

const COUNTER_KEY: &[u8] = b"counter";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let counter = Counter { value: 0 };
    deps.storage.set(COUNTER_KEY, &to_json_binary(&counter)?);
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let bz = deps.storage.get(COUNTER_KEY).unwrap();
    let mut counter: Counter = from_json(&bz).unwrap();
    match msg {
        ExecuteMsg::Count {} => {
            counter.value += 1;
        }
    };
    deps.storage.set(COUNTER_KEY, &to_json_binary(&counter)?);
    Ok(Response::new().add_attribute("method", "execute"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value {} => {
            let bz = deps.storage.get(COUNTER_KEY).unwrap();
            let counter: Counter = from_json(&bz).unwrap();
            return to_json_binary(&counter);
        }
        _ => unreachable!(),
    };
}
