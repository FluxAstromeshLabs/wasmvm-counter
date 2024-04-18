use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Count {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Counter)]
    Count {}
}

#[cw_serde]
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

    match msg {
        ExecuteMsg::Count{} => {
            let bz = deps.storage.get(COUNTER_KEY).unwrap();
            let mut counter: Counter = from_json(&bz).unwrap();
            counter.value += 1;
            deps.storage.set(COUNTER_KEY, &to_json_binary(&counter)?);
        },
    };
    Ok(Response::new().add_attribute("method", "execute"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Count{} => {
            let bz = deps.storage.get(COUNTER_KEY).unwrap();
            let counter: Counter = from_json(&bz).unwrap();
            return to_json_binary(&counter)
        }
    }
}
