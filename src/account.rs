struct Account {
    address: String,
    balance: u64,
    nonce: u64,
}
//Need to scan the chain for information about the account
impl Account {
    fn new(address: String) -> Self {
        Account {
            address,
            balance: 0,
            nonce: 0,
        }
    }

    fn update_balance(&mut self, amount: i64) {
        self.balance = (self.balance as i64 + amount) as u64;
    }

    // Increments the nonce
    fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    // Validates if a transaction can be processed
    fn validate_transaction(&self, transaction_amount: u64, transaction_nonce: u64) -> bool {
        self.balance >= transaction_amount && self.nonce == transaction_nonce
    }

    // Displays account information
    fn display_info(&self) {
        println!(
            "Address: {}\nBalance: {}\nNonce: {}",
            self.address, self.balance, self.nonce
        );
    }
}
