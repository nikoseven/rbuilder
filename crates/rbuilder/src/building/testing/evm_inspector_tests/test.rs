use reth_primitives::revm::env::tx_env_with_recovered;
use revm::{
    inspector_handle_register,
    primitives::{Env, ResultAndState, SpecId},
};

use crate::building::{
    evm_inspector::{RBuilderEVMInspector, UsedStateTrace},
    testing::test_chain_state::{BlockArgs, NamedAddr, TestChainState, TxArgs},
    BlockState,
};

use revm::DatabaseCommit;

#[test]
fn test_balance_change() -> eyre::Result<()> {
    // setup chain state
    let test_chain = TestChainState::new(BlockArgs::default())?;
    let block_building_context = test_chain.block_building_context();
    // setup db
    let state_provider = test_chain.provider_factory().latest()?;
    let mut block_state = BlockState::new(&state_provider);
    // mock transaction
    let tx = {
        // tx to transfer some etherss
        let tx_args = TxArgs::new(
            NamedAddr::User(1),
            block_state.nonce(test_chain.named_address(NamedAddr::User(1))?)?,
        )
        .to(NamedAddr::User(2))
        .value(100000);
        let tx = test_chain.sign_tx(tx_args)?;
        println!("Tx: {:?}", tx);
        tx
    };
    let tx = {
        let tx_args = TxArgs::new_send_to(
            NamedAddr::User(1),
            0,
            10000,
            test_chain.named_address(NamedAddr::User(2))?,
        );
        let tx = test_chain.sign_tx(tx_args)?;
        tx
    };
    // let tx = {
    //     let tx_args = TxArgs::new_increment_value(NamedAddr::User(1), 0, 1, 0);
    //     let tx = test_chain.sign_tx(tx_args)?;
    //     tx
    // };
    // mock evm inspector
    let mut used_state_trace = UsedStateTrace::default();
    let mut inspector = RBuilderEVMInspector::new(&tx, Some(&mut used_state_trace));
    // mock evm
    // mock evm :: setup env

    // TODO: fill env
    let mut db_ref = block_state.new_db_ref();
    let env = Env {
        cfg: block_building_context.initialized_cfg.cfg_env.clone(),
        block: block_building_context.block_env.clone(),
        tx: tx_env_with_recovered(&tx),
    };
    let mut evm = revm::Evm::builder()
        .with_spec_id(SpecId::SHANGHAI)
        .with_env(Box::new(env))
        .with_db(db_ref.as_mut())
        .with_external_context(&mut inspector)
        .append_handler_register(inspector_handle_register)
        .build();
    #[warn(path_statements)]
    // execute transaction
    let res = match evm.transact() {
        Ok(res) => {
            println!("Transact Ok");
            res
        }
        Err(err) => {
            println!("Error {}", err);
            return Err(err.into());
        }
    };
    let mut db_context = evm.into_context();
    let db = &mut db_context.evm.db;
    db.commit(res.state.clone());

    println!("ResultOfTransact: {:?}", res);
    // check state
    let latest_state_provider = test_chain.provider_factory().latest()?;
    let mut block_state = BlockState::new(&latest_state_provider);
    println!(
        "AddressOfUser1: {} Balance: {:?}",
        test_chain.named_address(NamedAddr::User(1))?,
        block_state.balance(test_chain.named_address(NamedAddr::User(1))?),
    );
    println!(
        "AddressOfUser2: {} Balance: ",
        test_chain.named_address(NamedAddr::User(2))?
    );
    println!("UsedStateTrace: {:?}", used_state_trace);
    Ok(())
}
