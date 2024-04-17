use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Count,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryMsg {}

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
        ExecuteMsg::Count => {
            counter.value += 1;
        },
    };
    deps.storage.set(COUNTER_KEY, &to_json_binary(&counter)?);
    Ok(Response::new().add_attribute("method", "execute"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let bz = deps.storage.get(COUNTER_KEY).unwrap();
    let counter: Counter = from_json(&bz).unwrap();
    to_json_binary(&counter)
}
