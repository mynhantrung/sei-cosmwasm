use cosmwasm_std::{
    from_binary,
    testing::{MockApi, MockQuerier, MockStorage},
    Api, BalanceResponse, BankQuery, BlockInfo, MemoryStorage, Storage, Timestamp,
};
use cw_multi_test::{
    App, AppBuilder, BankKeeper, FailingDistribution, FailingStaking, Module, Router, WasmKeeper,
};
use sei_cosmwasm::{DenomOracleExchangeRatePair, SeiMsg, SeiQueryWrapper};

use crate::module::SeiModule;

// Get balance
pub fn get_balance(
    app: &App<BankKeeper, MockApi, MemoryStorage, SeiModule, WasmKeeper<SeiMsg, SeiQueryWrapper>>,
    addr: String,
    denom: String,
) -> BalanceResponse {
    let arr = app.read_module(|router, api, storage| {
        router.bank.query(
            api,
            storage,
            &MockQuerier::default(),
            &BlockInfo {
                height: 0,
                time: Timestamp::from_seconds(0u64),
                chain_id: "test".to_string(),
            },
            BankQuery::Balance {
                address: addr,
                denom: denom,
            },
        )
    });
    from_binary(&arr.unwrap()).unwrap()
}

// Mock app
pub fn mock_app<F>(
    init_fn: F,
    rates: Vec<DenomOracleExchangeRatePair>,
) -> App<
    BankKeeper,
    MockApi,
    MockStorage,
    SeiModule,
    WasmKeeper<SeiMsg, SeiQueryWrapper>,
    FailingStaking,
    FailingDistribution,
>
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            SeiModule,
            WasmKeeper<SeiMsg, SeiQueryWrapper>,
            FailingStaking,
            FailingDistribution,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    let appbuilder: AppBuilder<
        BankKeeper,
        MockApi,
        MockStorage,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        FailingStaking,
        FailingDistribution,
    > = AppBuilder::new()
        .with_custom(SeiModule::new_with_oracle_exchange_rates(rates))
        .with_wasm::<SeiModule, WasmKeeper<SeiMsg, SeiQueryWrapper>>(WasmKeeper::new());

    appbuilder.build(init_fn)
}