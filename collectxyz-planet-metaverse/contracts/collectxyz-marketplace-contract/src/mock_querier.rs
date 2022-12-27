use std::collections::HashMap;

use collectxyz::nft::{QueryMsg as XyzQueryMsg, XyzTokenInfo};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult,
    QueryRequest, StdError, SystemError, SystemResult, WasmQuery,
};
use terra_cosmwasm::TerraQueryWrapper;

pub fn mock_dependencies_custom(
    xyz_balances: HashMap<String, XyzTokenInfo>,
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, CustomMockQuerier> {
    let custom_querier = CustomMockQuerier::new(
        MockQuerier::<TerraQueryWrapper>::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]),
        xyz_balances,
    );
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct CustomMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
    xyz_balances: HashMap<String, XyzTokenInfo>,
}

impl CustomMockQuerier {
    pub fn new(
        base: MockQuerier<TerraQueryWrapper>,
        xyz_balances: HashMap<String, XyzTokenInfo>,
    ) -> Self {
        CustomMockQuerier { base, xyz_balances }
    }
}

impl<'a> CustomMockQuerier {
    pub fn update_xyz_balances(&'a mut self, xyz_balances: HashMap<String, XyzTokenInfo>) {
        self.xyz_balances = xyz_balances;
    }

    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr == "xyz-nft-contract" {
                    if let XyzQueryMsg::XyzNftInfo { token_id } =
                        from_binary::<XyzQueryMsg>(&msg).unwrap()
                    {
                        return self
                            .xyz_balances
                            .get(&token_id)
                            .map(|xyz| SystemResult::Ok(ContractResult::from(to_binary(xyz))))
                            .unwrap_or(SystemResult::Ok(ContractResult::from(Err(
                                StdError::not_found("xyz"),
                            ))));
                    } else {
                        panic!("unsupported message type! {}", msg)
                    }
                }
                panic!("unsupported query");
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl Querier for CustomMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}
