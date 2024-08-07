use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_provider::{ProviderBuilder, RootProvider};

use crate::utils::{http_provider, BoxedProvider};

#[test]
fn test_balance_change() -> eyre::Result<()> {
    let mut t = TestSetup::new();

    Ok(())
}

struct TestSetup {
    anvil: AnvilInstance,
    provider: BoxedProvider,
}

impl TestSetup {
    fn new() -> TestSetup {
        let anvil = Anvil::new()
            .block_time(1)
            .chain_id(1337)
            .try_spawn()
            .unwrap();
        let provider = http_provider(anvil.endpoint_url());
        TestSetup { anvil, provider }
    }
}
