#![allow(warnings)]

extern crate chapa_rust;
use chapa_rust::Transaction;

fn main() {
    chapa_rust::get_banks();

    let test_transaction = Transaction {
        amount: 150,
        currency: String::from("USD"),
        email: String::from("john_doe@gmail.com"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        tx_ref: String::from("mail_order_injera"),
    };

    chapa_rust::initialize_transaction(test_transaction);

    chapa_rust::verify_transaction(String::from("mail_order_injera"));
}
