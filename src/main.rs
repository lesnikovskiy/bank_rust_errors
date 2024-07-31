#[derive(Debug)]
struct Account {
    balance: u32,
    id: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

fn main() {
    println!("Hello, world!");
}
