#![cfg(test)]
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_slice, to_binary, Coin, ContractResult, OwnedDeps, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult, WasmQuery,
};
use terra_cosmwasm::TerraQueryWrapper;

pub fn mock_dependencies_custom(
    contract_balance: &[Coin]
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier = WasmMockQuerier::new(
        MockQuerier::<TerraQueryWrapper>::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]),
    );
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>
}

impl Querier for WasmMockQuerier {
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

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr.starts_with("seed") {
                    return SystemResult::Ok(ContractResult::from(to_binary(&msg)));
                }
                panic!("unsupported query message type");
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new( base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier {  base }
    }
}
