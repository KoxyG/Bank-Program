// Bank program
// create-account
// deposit
// withdraw
// transfer
// balance

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bank {
    account_number: i32,
    account_name: String,
    account_balance: u32,
}

impl Bank {
    fn create_account(account_number: i32, account_name: String, account_balance: u32) -> Bank {
        Bank {
            account_number,
            account_name,
            account_balance,
        }
    }

    fn deposit(&mut self, amount: u32) {
        self.account_balance += amount;
    }

    fn withdraw(&mut self, amount: u32) -> Option<u32> {
        if amount > self.account_balance {
            println!("Insufficient balance");
            None
        } else {
            self.account_balance -= amount;
            Some(amount)
        }
    }

    fn balance(&self) -> u32 {
        self.account_balance
    }

    fn transfer(&mut self, amount: u32, account: &mut Bank) -> Option<u32> {
        if amount > self.account_balance {
            println!("Insufficient balance for transfer");
            None
        } else {
            self.account_balance -= amount;
            account.account_balance += amount;
            Some(amount)
        }
    }

    fn account_details(&self) -> (i32, &str, u32) {
        (self.account_number, &self.account_name, self.account_balance)
    }
}

fn main() {
    
    // create account
    let mut account1 = Bank::create_account(1234567, "Koxy".to_string(), 300);
    println!("New user Open account with ${:?}", account1.balance());
    
    // Depoosit amount and check balance
   account1.deposit(1000);
    println!("Balance after making a deposit  is ${}", account1.balance());

    // Check account 1 details
    let (number, name, balance) = account1.account_details();
    println!("Account 1 details: Number: {}, Name: {}, Balance: ${}", number, name, balance);

    // Create new account
    let mut account2 = Bank::create_account(987654321, "Progress".to_string(), 500);
    println!("New user Open account balance ${:?}", account2.balance());
    println!("Account 2 details {:?}", account2.account_details());


    // Before transfer
    println!("Before transfer:");
    println!("Account 1 balance: ${}", account1.balance());
    println!("Account 2 balance: ${}", account2.balance());
     

    // Perform transfer
    let transfer_amount = 500;
    match account1.transfer(transfer_amount, &mut account2) {
        Some(amount) => println!("Successfully transferred ${}", amount),
        None => println!("Transfer failed"),
    }

    println!("After transfer:");
    println!("Account 1 balance: ${}", account1.balance());
    println!("Account 2 balance: ${}", account2.balance())


}
