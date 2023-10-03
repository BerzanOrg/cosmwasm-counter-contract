use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_count: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Decrement {},
    Reset {},
    Set { new_count: u32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CountResp)]
    Count {},
}

#[cw_serde]
pub struct CountResp {
    pub count: u32,
}
