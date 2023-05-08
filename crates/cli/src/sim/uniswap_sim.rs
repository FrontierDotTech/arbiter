#![warn(missing_docs)]
use std::error::Error;

use bindings::{
    arbiter_token,
    weth9,
};
use bytes::Bytes;
use ethers::{
    abi::Token,
    prelude::{BaseContract, U256},
    types::{H160, H256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, B160};
use simulate::{
    agent::{user::User, Agent, AgentType},
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::recast_address,
};


/// Run a simulation.
pub fn uniswap_sim() -> Result<(), Box<dyn Error>> {
    // define the wad constant
    let decimals = 18_u8;
    let wad: U256 = U256::from(10_i64.pow(decimals as u32));
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();

    let user_name = "arbitrageur";
    let user_address = B160::from_low_u64_be(2);
    let arbitrageur = User::new(user_name, None);

    manager.activate_agent(AgentType::User(arbitrageur), user_address)?;
    let _arbitrageur = manager.agents.get(user_name).unwrap();
    println!("Arbitrageur created at: {}", user_address);
    let _admin = manager.agents.get("admin").unwrap();

    // Deploying Contracts
    let contracts = deploy_uniswap_sim_contracts(&mut manager, wad)?;

    uniswap_sim_intitalization_calls(&mut manager, contracts, decimals)?;

    Ok(())
}

/// Deploy the contracts to the simulation environment.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `wad` - Wad constant to use for the simulation. (U256)
/// # Returns
/// * `SimulationContracts` - Contracts deployed to the simulation environment. (SimulationContracts)
fn deploy_uniswap_sim_contracts(
    manager: &mut SimulationManager,
    wad: U256,
) -> Result<SimulationContract<IsDeployed>, Box<dyn Error>> {
    let decimals = 18_u8;
    let admin = manager.agents.get("admin").unwrap();
    // Deploy Weth
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = weth.deploy(&mut manager.environment, admin, ());
    println!("WETH deployed at: {}", weth.address);
    Ok(weth)

    // Deploy the registry contract.
}

/// Calls the initialization functions of each contract.
/// # Arguments
/// * `manager` - Simulation manager to deploy contracts to. (SimulationManager)
/// * `contracts` - Contracts deployed to the simulation environment. (SimulationContracts)
/// * `decimals` - Decimals to use for the simulation. (u8)
fn uniswap_sim_intitalization_calls(
    manager: &mut SimulationManager,
    contracts: SimulationContract<IsDeployed>,
    decimals: u8,
) -> Result<(), Box<dyn Error>> {

    Ok(())
}