#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: vec![],
        }
    }
}

fn print_accounts(bank: &Bank) {
    println!("{:#?}", bank.accounts);
}

fn change_account(account: &mut Account) {
    account.balance = 100;
}

fn hack_bank(bank: &mut Bank) {
    bank.accounts.push(Account::new(666, String::from("Satan")));
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("Ruslan Lesnikovskiy"));
    let account2 = Account::new(2, String::from("Vasya"));
    bank.accounts.push(account1);
    bank.accounts.push(account2);

    hack_bank(&mut bank);

    print_accounts(&bank);
    print_accounts(&bank);

    let mut account = Account::new(3, String::from("Trump"));
    change_account(&mut account);
    println!("{:#?}", account);

    let mut monobank = Bank::new();
    let account = Account::new(299, String::from("Genadiy Moskalenko"));
    add_account(&mut monobank, account);
    println!("{:#?}", monobank);
}
