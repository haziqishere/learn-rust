// References and Borrowing
// Safety and Performance

// What is References?
// Enable to borrow values without taking ownership by holding physical address of the original value
// 2 Types:
// Immutable Reference
// Mutable Reference

// fn main() {
//     // Immutable Reference
//     let x: i32 = 5;
//     let r = &x;
//     // *r += 1; // will raise "cannot assign to `*r`, which is behind a `&` reference"
//     println!("value of x is {}", x);
//     println!("value of r is {}", r);

//     // Mutable Reference
//     let mut y: i32 = 5;
//     let g: &mut i32 = &mut y;

//     *g += 1;

//     // println!("value of y is {}", y); cause error "cannot borrow `y` as immutable because it is also
//     //                                               borrowed as mutable immutable borrow occurs here"
//     println!("value of g is {}", g);
// }

fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withraw money
    account.withdraw(45.5);
}

// Demonstration on one mutable reference or many immutable references
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withrawing {} from account ownened by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        )
    }
}
