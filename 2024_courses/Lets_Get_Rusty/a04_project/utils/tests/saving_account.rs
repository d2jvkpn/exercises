mod help;

use utils::SavingsAccount;

#[test]
fn should_have_a_starting_balance_of_0() {
    help::common_setup();

    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0.0);
}
