use std::thread;
use std::time::Duration;

fn main() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread1.join().unwrap();

    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &mut Bank, amount: f32) {
        the_bank.balance -= amount;
    }

    let mut bank: Bank = Bank { balance: 100.0 };
    assert!(bank.balance == 100.0);
    withdraw(&mut bank, 5.00);
    assert!(bank.balance == 95.0);
    println!("bank balance: {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.00);
    }
    thread::spawn(move ||{
        customer(&mut bank);
    }).join().unwrap();
}
