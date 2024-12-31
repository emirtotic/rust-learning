fn main() {

    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 155.55,
    };

    // Immutable borrow to check the ballance

    account.check_balance();
    account.withdraw(20.53);
    account.check_balance();

}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {

    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has the balance of {}", self.owner, self.balance);
    }
}