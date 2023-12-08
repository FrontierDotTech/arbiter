use super::*;

#[tokio::test]
async fn arbiter_math() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let arbiter_math = deploy_arbiter_math(client).await.unwrap();

    // Test the cdf function
    let cdf_output = arbiter_math
        .cdf(ethers::types::I256::from(1))
        .call()
        .await
        .unwrap();
    println!("cdf(1) = {}", cdf_output);
    assert_eq!(cdf_output, ethers::types::I256::from(500000000000000000u64));

    // Test the pdf function
    let pdf_output = arbiter_math
        .pdf(ethers::types::I256::from(1))
        .call()
        .await
        .unwrap();
    println!("pdf(1) = {}", pdf_output);
    assert_eq!(pdf_output, ethers::types::I256::from(398942280401432678u64));

    // Test the ppf function.
    let ppf_output = arbiter_math
        .ppf(ethers::types::I256::from(1))
        .call()
        .await
        .unwrap();
    println!("ppf(1) = {}", ppf_output);
    assert_eq!(
        ppf_output,
        ethers::types::I256::from(-8710427241990476442_i128)
    );

    // Test the mulWadDown function.
    let mulwaddown_output = arbiter_math
        .mul_wad_down(
            ethers::types::U256::from(1_000_000_000_000_000_000_u128),
            ethers::types::U256::from(2),
        )
        .call()
        .await
        .unwrap();
    println!("mulWadDown(1, 2) = {}", mulwaddown_output);
    assert_eq!(mulwaddown_output, ethers::types::U256::from(2));

    // Test the mulWadUp function.
    let mulwadup_output = arbiter_math
        .mul_wad_up(
            ethers::types::U256::from(1_000_000_000_000_000_000_u128),
            ethers::types::U256::from(2),
        )
        .call()
        .await
        .unwrap();
    println!("mulWadUp(1, 2) = {}", mulwadup_output);
    assert_eq!(mulwadup_output, ethers::types::U256::from(2));

    // Test the divWadDown function.
    let divwaddown_output = arbiter_math
        .div_wad_down(
            ethers::types::U256::from(1_000_000_000_000_000_000_u128),
            ethers::types::U256::from(2),
        )
        .call()
        .await
        .unwrap();
    println!("divWadDown(1, 2) = {}", divwaddown_output);
    assert_eq!(
        divwaddown_output,
        ethers::types::U256::from(500000000000000000000000000000000000_u128)
    );

    // Test the divWadUp function.
    let divwadup_output = arbiter_math
        .div_wad_up(
            ethers::types::U256::from(1_000_000_000_000_000_000_u128),
            ethers::types::U256::from(2),
        )
        .call()
        .await
        .unwrap();
    println!("divWadUp(1, 2) = {}", divwadup_output);
    assert_eq!(
        divwadup_output,
        ethers::types::U256::from(500000000000000000000000000000000000_u128)
    );

    // Test the lnWad function.
    let lnwad_output = arbiter_math
        .log(ethers::types::I256::from(1_000_000_000_000_000_000_u128))
        .call()
        .await
        .unwrap();
    println!("ln(1) = {}", lnwad_output);
    assert_eq!(lnwad_output, ethers::types::I256::from(0));

    // Test the sqrt function
    let sqrt_output = arbiter_math
        .sqrt(ethers::types::U256::from(1_000_000_000_000_000_000_u128))
        .call()
        .await
        .unwrap();
    println!("sqrt(1) = {}", sqrt_output);
    assert_eq!(sqrt_output, ethers::types::U256::from(1_000_000_000));
}

// TODO: It would be good to change this to `token_functions` and test all
// relevant ERC20 functions (e.g., transfer, approve, etc.).
#[tokio::test]
async fn token_mint_and_balance() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let arbx = deploy_arbx(client.clone()).await.unwrap();

    // Mint some tokens to the client.
    arbx.mint(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    )
    .send()
    .await
    .unwrap()
    .await
    .unwrap();

    // Fetch the balance of the client.
    let balance = arbx
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();

    // Check that the balance is correct.
    assert_eq!(balance, ethers::types::U256::from(TEST_MINT_AMOUNT));
}

#[tokio::test]
async fn liquid_exchange_swap() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let (arbx, arby, liquid_exchange) = deploy_liquid_exchange(client.clone()).await.unwrap();

    // Mint tokens to the client then check balances.
    arbx.mint(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    )
    .send()
    .await
    .unwrap()
    .await
    .unwrap();
    arby.mint(
        client.default_sender().unwrap(),
        ethers::types::U256::from(TEST_MINT_AMOUNT),
    )
    .send()
    .await
    .unwrap()
    .await
    .unwrap();
    let arbx_balance = arbx
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    let arby_balance = arby
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    println!("arbx_balance prior to swap = {}", arbx_balance);
    println!("arby_balance prior to swap = {}", arby_balance);
    assert_eq!(arbx_balance, ethers::types::U256::from(TEST_MINT_AMOUNT));
    assert_eq!(arby_balance, ethers::types::U256::from(TEST_MINT_AMOUNT));

    // Get the price at the liquid exchange
    let price = liquid_exchange.price().call().await.unwrap();
    println!("price in 18 decimal WAD: {}", price);

    // Mint tokens to the liquid exchange.
    let exchange_mint_amount = ethers::types::U256::MAX / 2;
    arbx.mint(liquid_exchange.address(), exchange_mint_amount)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();
    arby.mint(liquid_exchange.address(), exchange_mint_amount)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // Approve the liquid exchange to spend the client's tokens.
    arbx.approve(liquid_exchange.address(), ethers::types::U256::MAX)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();
    arby.approve(liquid_exchange.address(), ethers::types::U256::MAX)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // Swap some X for Y on the liquid exchange.
    let swap_amount_x = ethers::types::U256::from(TEST_MINT_AMOUNT) / 2;
    liquid_exchange
        .swap(arbx.address(), swap_amount_x)
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    // Check the client's balances are correct.
    let arbx_balance_after_swap_x = arbx
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    let arby_balance_after_swap_x = arby
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    println!("arbx_balance after swap = {}", arbx_balance_after_swap_x);
    println!("arby_balance after swap = {}", arby_balance_after_swap_x);
    assert_eq!(
        arbx_balance_after_swap_x,
        ethers::types::U256::from(TEST_MINT_AMOUNT) - swap_amount_x
    );
    let additional_y = swap_amount_x * price / ethers::types::U256::from(10_u64.pow(18));
    assert_eq!(
        arby_balance_after_swap_x,
        ethers::types::U256::from(TEST_MINT_AMOUNT) + additional_y
    );

    // Swap some Y for X on the liquid exchange.
    let swap_amount_y = additional_y;
    liquid_exchange
        .swap(arby.address(), swap_amount_y)
        .send()
        .await
        .unwrap()
        .await
        .unwrap();

    // Check the client's balances are correct.
    let arbx_balance_after_swap_y = arbx
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    let arby_balance_after_swap_y = arby
        .balance_of(client.default_sender().unwrap())
        .call()
        .await
        .unwrap();
    println!("arbx_balance after swap = {}", arbx_balance_after_swap_y);
    println!("arby_balance after swap = {}", arby_balance_after_swap_y);

    // The balance here is off by one due to rounding and the extremely small
    // balances we are using.
    assert_eq!(
        arbx_balance_after_swap_y,
        ethers::types::U256::from(TEST_MINT_AMOUNT) - 1
    );
    assert_eq!(
        arby_balance_after_swap_y,
        ethers::types::U256::from(TEST_MINT_AMOUNT)
    );
}

#[tokio::test]
async fn price_simulation_oracle() {
    let (_manager, client) = startup_user_controlled().unwrap();
    let (.., liquid_exchange) = deploy_liquid_exchange(client.clone()).await.unwrap();

    let price_path = vec![
        1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0, 8000.0,
    ];

    // Get the initial price of the liquid exchange.
    let initial_price = liquid_exchange.price().call().await.unwrap();
    assert_eq!(initial_price, parse_ether(LIQUID_EXCHANGE_PRICE).unwrap());

    for price in price_path {
        let wad_price = parse_ether(price).unwrap();
        liquid_exchange
            .set_price(wad_price)
            .send()
            .await
            .unwrap()
            .await
            .unwrap();
        let new_price = liquid_exchange.price().call().await.unwrap();
        assert_eq!(new_price, wad_price);
    }
}
