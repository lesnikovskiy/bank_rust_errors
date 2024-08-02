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

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    // fn total_balance(&mut self) -> i32 {
    //     let mut total = 0;
    //     for acc in self.accounts {
    //         total += acc.balance;
    //     }
    //     total
    // }
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    bank.add_account(account);

    println!("{:#?}", bank);

    let mut acc = Account::new(2, String::from("Jason"));
    acc.deposit(999);

    println!("{:#?}", acc);

    acc.withdraw(555);
    println!("{:#?}", acc);
}
