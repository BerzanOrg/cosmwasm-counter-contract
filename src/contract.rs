use crate::msg::CountResp;
use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::COUNT,
};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let starting_count = msg.starting_count;

    COUNT.save(deps.storage, &starting_count)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Count {} => to_binary(&query::count(deps)?),
    }
}

mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<CountResp> {
        let count = COUNT.load(deps.storage)?;
        let resp = CountResp { count };
        Ok(resp)
    }
}

#[allow(dead_code)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment {} => exec::increment(deps, info),
        ExecuteMsg::Decrement {} => exec::decrement(deps, info),
        ExecuteMsg::Reset {} => exec::reset(deps, info),
        ExecuteMsg::Set { new_count } => exec::set(deps, info, new_count),
    }
}

mod exec {
    use super::*;

    pub fn increment(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let old_count = COUNT.load(deps.storage)?;
        let new_count = old_count + 1;

        let event = Event::new("incremented").add_attribute("addr", info.sender);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "increment")
            .add_attribute("old_count", old_count.to_string())
            .add_attribute("new_count", new_count.to_string());

        COUNT.save(deps.storage, &new_count)?;

        Ok(resp)
    }

    pub fn decrement(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let old_count = COUNT.load(deps.storage)?;
        let new_count = old_count - 1;

        let event = Event::new("decremented").add_attribute("addr", info.sender);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "decrement")
            .add_attribute("old_count", old_count.to_string())
            .add_attribute("new_count", new_count.to_string());

        COUNT.save(deps.storage, &new_count)?;

        Ok(resp)
    }
    pub fn reset(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let old_count = COUNT.load(deps.storage)?;
        let new_count = 0;

        let event = Event::new("reset").add_attribute("addr", info.sender);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "reset")
            .add_attribute("old_count", old_count.to_string())
            .add_attribute("new_count", new_count.to_string());

        COUNT.save(deps.storage, &new_count)?;

        Ok(resp)
    }
    pub fn set(deps: DepsMut, info: MessageInfo, new_count: u32) -> StdResult<Response> {
        let old_count = COUNT.load(deps.storage)?;

        let event = Event::new("set").add_attribute("addr", info.sender);

        let resp = Response::new()
            .add_event(event)
            .add_attribute("action", "set")
            .add_attribute("old_count", old_count.to_string())
            .add_attribute("new_count", new_count.to_string());

        COUNT.save(deps.storage, &new_count)?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn instantiation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: CountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Count {})
            .unwrap();

        assert_eq!(resp, CountResp { count: 7 });
    }

    #[test]
    fn query_count() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: CountResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Count {})
            .unwrap();

        assert_eq!(7, resp.count);
    }

    #[test]
    fn exec_increment() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::Increment {},
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            resp.events
                .iter()
                .find(|ev| ev.ty == "wasm-incremented")
                .unwrap()
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "increment"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "old_count")
                .unwrap()
                .value,
            "7"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "new_count")
                .unwrap()
                .value,
            "8"
        );
    }

    #[test]
    fn exec_decrement() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::Decrement {},
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            resp.events
                .iter()
                .find(|ev| ev.ty == "wasm-decremented")
                .unwrap()
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "decrement"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "old_count")
                .unwrap()
                .value,
            "7"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "new_count")
                .unwrap()
                .value,
            "6"
        );
    }

    #[test]
    fn exec_reset() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(Addr::unchecked("user"), addr, &ExecuteMsg::Reset {}, &[])
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            resp.events
                .iter()
                .find(|ev| ev.ty == "wasm-reset")
                .unwrap()
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "reset"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "old_count")
                .unwrap()
                .value,
            "7"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "new_count")
                .unwrap()
                .value,
            "0"
        );
    }

    #[test]
    fn exec_set() {
        let new_count = 888;

        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { starting_count: 7 },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::Set { new_count },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            resp.events
                .iter()
                .find(|ev| ev.ty == "wasm-set")
                .unwrap()
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "set"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "old_count")
                .unwrap()
                .value,
            "7"
        );
        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "new_count")
                .unwrap()
                .value,
            new_count.to_string(),
        );
    }
}
