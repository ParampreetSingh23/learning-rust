
struct BankAccount {
    owner: String,
    balance: i32,
}

fn show_balance(account: &BankAccount) {
    println!("{}" , account.balance);
}

fn deposit(account: &mut BankAccount, amount: i32) {
     account.balance = account.balance + amount;
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Alice"),
        balance: 1000,
    };

    show_balance(&account);

    deposit(&mut account, 500);

    show_balance(&account);
}
  