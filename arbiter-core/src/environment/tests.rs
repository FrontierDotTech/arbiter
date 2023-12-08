use super::*;
use crate::middleware::RevmMiddleware;

pub(crate) const TEST_ENV_LABEL: &str = "test";

#[test]
fn auto_start_on_build() {
    let environment: Environment<InMemoryDB> = EnvironmentBuilder::new().build();
    let _client = RevmMiddleware::new(&environment, Some(TEST_ENV_LABEL)).unwrap();
}

#[test]
fn new_with_builder() {
    let environment: Environment<InMemoryDB> = EnvironmentBuilder::new().build();
    assert_eq!(environment.parameters.label, None);
}
#[test]
fn new_with_builder_custom_settings() {
    let environment: Environment<InMemoryDB> = EnvironmentBuilder::new()
        .label(TEST_ENV_LABEL)
        .block_settings(BlockSettings::RandomlySampled {
            block_rate: 1.0,
            block_time: 12,
            seed: 1,
        })
        .gas_settings(GasSettings::RandomlySampled { multiplier: 1.0 })
        .build();
    assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
}
#[test]
fn new_user_controlled() {
    let params = EnvironmentParameters {
        label: Some(TEST_ENV_LABEL.to_string()),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    };
    let environment: Environment<InMemoryDB> = Environment::new(params, None);
    assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
}

#[test]
fn new_randomly_sampled() {
    let block_type = BlockSettings::RandomlySampled {
        block_rate: 1.0,
        block_time: 12,
        seed: 1,
    };
    let params = EnvironmentParameters {
        label: Some(TEST_ENV_LABEL.to_string()),
        block_settings: block_type,
        gas_settings: GasSettings::RandomlySampled { multiplier: 1.0 },
    };
    let environment: Environment<InMemoryDB> = Environment::new(params, None);
    assert_eq!(environment.parameters.label, Some(TEST_ENV_LABEL.into()));
}

#[test]
fn run() {
    let params = EnvironmentParameters {
        label: Some(TEST_ENV_LABEL.to_string()),
        block_settings: BlockSettings::UserControlled,
        gas_settings: GasSettings::UserControlled,
    };
    Environment::<InMemoryDB>::new(params, None);
}

#[test]
fn conversion() {
    // Test with a value that fits in u64.
    let input = U256::from(10000);
    assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(10000));

    // Test with a value that is exactly at the limit of u64.
    let input = U256::from(u64::MAX);
    assert_eq!(convert_uint_to_u64(input).unwrap(), U64::from(u64::MAX));

    // Test with a value that exceeds the limit of u64.
    let input = U256::from(u64::MAX) + U256::from(1);
    assert!(convert_uint_to_u64(input).is_err());
}
