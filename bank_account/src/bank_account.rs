#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.00 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount <= self.balance && amount > 0.00 {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let acct = BankAccount::new(200.00);
        assert_eq!(acct.balance(), (200.00));
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut acct = BankAccount::new(200.00);
        acct.deposit(20.00);
        assert!((acct.balance() - 220.00).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut acct = BankAccount::new(200.00);
        acct.withdraw(20.00);
        assert_eq!(acct.balance(), 180.00);
    }

    // Add more tests here
    #[test]
    fn test_deposit_negative() {
        let mut acct = BankAccount::new(200.00);
        acct.deposit(-20.00);
        assert_eq!(acct.balance(), 200.00);
    }

    #[test]
    fn test_withdraw_negative() {
        let mut acct = BankAccount::new(200.00);
        acct.withdraw(-20.00);
        assert_eq!(acct.balance(), 200.00);
    }

    #[test]
    fn test_withdraw_overdraw() {
        let mut acct = BankAccount::new(200.00);
        acct.withdraw(210.00);
        assert_eq!(acct.balance(), 200.00);
    }
}