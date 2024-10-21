mod bank_account;


fn main() {
    let bank_account = bank_account::BankAccount::new(100.00);
    println!("{:?}", bank_account);
}