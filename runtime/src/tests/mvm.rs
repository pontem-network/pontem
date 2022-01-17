/// Test balances in Runtime. Mostly currencies, balances, vesting functional.
use crate::tests::mock::*;
use frame_support::{assert_ok, assert_err_ignore_postinfo, dispatch::DispatchError};
use orml_traits::currency::MultiCurrency;

pub mod modules {
    use assets::Asset;
    pub static BANK: Asset = Asset::new(
        "Bank",
        "src/tests/assets/user/build/assets/bytecode_modules/Bank.mv",
    );
}

pub mod transactions {
    use assets::Asset;
    pub static TRANSFER_PONT: Asset = Asset::new(
        "transfer",
        "src/tests/assets/user/build/assets/transaction/transfer_pont.mvt",
    );
    pub static TRANSFER_KSM: Asset = Asset::new(
        "transfer",
        "src/tests/assets/user/build/assets/transaction/transfer_ksm.mvt",
    );
    pub static DEPOSIT_BANK_PONT: Asset = Asset::new(
        "deposit_bank_pont",
        "src/tests/assets/user/build/assets/transaction/deposit_bank_pont.mvt",
    );
    pub static DEPOSIT_BANK_KSM: Asset = Asset::new(
        "deposit_bank_ksm",
        "src/tests/assets/user/build/assets/transaction/deposit_bank_ksm.mvt",
    );
}

#[test]
/// Transfer native balance to bank and be sure that total supply doesn't change.
fn transfer_balance_to_bank() {
    let currency_id = GetNativeCurrencyId::get();

    const GAS_LIMIT: u64 = 1_000_000;
    let initial_balance = to_unit(100, currency_id);

    // Transfer script transfers 50 PONT to bank.
    let to_spent = to_unit(50, currency_id);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::BOB.account(),
            CurrencyId::PONT,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            // Get initial supply.
            let b_total_supply = Balances::total_issuance();
            let c_total_supply = Currencies::total_issuance(currency_id);

            // Publish Bank module.
            assert_ok!(Mvm::publish_module(
                Origin::signed(Accounts::BOB.account()),
                modules::BANK.bytes().to_vec(),
                GAS_LIMIT
            ));

            // Transfer.
            assert_ok!(Mvm::execute(
                Origin::signed(Accounts::BOB.account()),
                transactions::DEPOSIT_BANK_PONT.bytes().to_vec(),
                GAS_LIMIT
            ));

            // Check total issuances.
            assert_eq!(Balances::total_issuance(), b_total_supply,);

            assert_eq!(Currencies::total_issuance(currency_id), c_total_supply,);

            // Check sender balance.
            assert_eq!(
                Balances::free_balance(Accounts::BOB.account()),
                initial_balance - to_spent
            );
        });
}

#[test]
/// Transfer tokens to bank and be sure that total supply doesn't change.
fn transfer_tokens_to_bank() {
    let currency_id = CurrencyId::KSM;

    const GAS_LIMIT: u64 = 1_000_000;
    let initial_balance = to_unit(100, currency_id);

    // Transfer script transfers 50 KSM to bank.
    let to_spent = to_unit(50, currency_id);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::BOB.account(),
            CurrencyId::KSM,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            // Get initial supply.
            let total_supply = Currencies::total_issuance(currency_id);

            // Publish Bank module.
            assert_ok!(Mvm::publish_module(
                Origin::signed(Accounts::BOB.account()),
                modules::BANK.bytes().to_vec(),
                GAS_LIMIT
            ));

            // Transfer.
            assert_ok!(Mvm::execute(
                Origin::signed(Accounts::BOB.account()),
                transactions::DEPOSIT_BANK_KSM.bytes().to_vec(),
                GAS_LIMIT
            ));

            assert_eq!(Currencies::total_issuance(currency_id), total_supply,);

            // Check sender balance.
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                initial_balance - to_spent
            );
        });
}

#[test]
/// Transfer native balance, be sure balance changed and check that vested balance not used, etc.
fn transfer_vested_balance_fails() {
    let currency_id = GetNativeCurrencyId::get();

    const GAS_LIMIT: u64 = 1_000_000;
    let initial_balance = to_unit(100, CurrencyId::PONT);
    let start_vesting = 10;
    let duration: u32 = 100;

    // We reduce free balance till 40 PONT, so sending 50 PONT should return error.
    let free_balance = to_unit(40, CurrencyId::PONT);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::BOB.account(),
            currency_id,
            initial_balance,
        )])
        .set_vesting(vec![(
            Accounts::BOB.account(),
            start_vesting,
            duration,
            free_balance,
        )])
        .build()
        .execute_with(|| {
            let supply = Balances::total_issuance();
            // Should error, because usable balance for Bob is 40 PONT
            assert_err_ignore_postinfo!(
                Mvm::execute(
                    Origin::signed(Accounts::BOB.account()),
                    transactions::TRANSFER_PONT.bytes().to_vec(),
                    GAS_LIMIT
                ),
                DispatchError::Module {
                    index: 67,
                    error: 153,
                    message: Some("Aborted")
                },
            );

            // Check balance is not changed.
            assert_eq!(
                Balances::free_balance(Accounts::BOB.account()),
                initial_balance,
            );

            // Check supply is not changed.
            assert_eq!(Balances::total_issuance(), supply,);
        });
}

#[test]
// Transfer native balance from account to account.
fn transfer_balance() {
    let currency_id = GetNativeCurrencyId::get();

    const GAS_LIMIT: u64 = 1_000_000;
    let initial_balance = to_unit(100, currency_id);

    // Transfer script transfers 50 PONT .
    let to_spent = to_unit(50, currency_id);

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::BOB.account(),
            currency_id,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            // Get initial supply.
            let b_total_supply = Balances::total_issuance();
            let c_total_supply = Currencies::total_issuance(currency_id);

            // Transfer.
            assert_ok!(Mvm::execute(
                Origin::signed(Accounts::BOB.account()),
                transactions::TRANSFER_PONT.bytes().to_vec(),
                GAS_LIMIT
            ));

            // Check total issuances.
            assert_eq!(Balances::total_issuance(), b_total_supply,);

            assert_eq!(Currencies::total_issuance(currency_id), c_total_supply,);

            // Check sender balance.
            assert_eq!(
                Balances::free_balance(Accounts::BOB.account()),
                initial_balance - to_spent
            );
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                initial_balance - to_spent
            );

            // Check alice balance.
            assert_eq!(Balances::free_balance(Accounts::ALICE.account()), to_spent);
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                initial_balance - to_spent
            );
        });
}

#[test]
/// Transfer tokens from account to account.
fn transfer_tokens() {
    let currency_id = CurrencyId::KSM;

    const GAS_LIMIT: u64 = 1_000_000;
    let initial_balance = to_unit(10000000, currency_id);
    eprintln!("init b: {}", initial_balance);

    // Transfer script transfers 0.5 KSM.
    let to_spent = 500000000000;

    RuntimeBuilder::new()
        .set_balances(vec![(
            Accounts::BOB.account(),
            currency_id,
            initial_balance,
        )])
        .build()
        .execute_with(|| {
            // Get initial supply.
            let total_supply = Currencies::total_issuance(currency_id);

            // Transfer.
            assert_ok!(Mvm::execute(
                Origin::signed(Accounts::BOB.account()),
                transactions::TRANSFER_KSM.bytes().to_vec(),
                GAS_LIMIT
            ));

            // Check total issuances.
            assert_eq!(Currencies::total_issuance(currency_id), total_supply,);

            // Check sender balance.
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::BOB.account()),
                initial_balance - to_spent
            );

            // Check Alice balance.
            assert_eq!(
                Currencies::free_balance(currency_id, &Accounts::ALICE.account()),
                to_spent
            );
        });
}
