mod help;

use utils::SavingsAccount;

#[test]
fn t_savings_account() {
    help::common_setup();

    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0.0);
}
