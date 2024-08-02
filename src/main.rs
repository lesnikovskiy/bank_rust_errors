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

    fn setBalance(&mut self, balance: i32) {
        self.balance = balance;
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

    fn addAccount(&mut self, account: Account) {
        self.accounts.push(account);
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

fn print_bank(bank: &Bank) {
    println!("{:#?}", bank);
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
    let mut account = Account::new(299, String::from("Genadiy Moskalenko"));
    account.setBalance(999);

    add_account(&mut monobank, account);

    println!("{:#?}", monobank);

    let mut privat = Bank::new();
    let marina_acc = Account::new(12, String::from("Marina Buzova"));
    privat.addAccount(marina_acc);

    print_bank(&privat);
}
