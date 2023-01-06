#![allow(warnings)]

extern crate chapa_rust;
use chapa_rust::Transaction;

fn main() {
    //chapa_rust::get_banks();

    let test_transaction = Transaction {
        amount: 150,
        currency: String::from("ETB"),
        email: String::from("kvaradona@gmail.com"),
        first_name: String::from("Khvicha"),
        last_name: String::from("Kvaratskhelia"),
        tx_ref: String::from("kvaradona_napoli_salary_2"),
    };

    let test_transaction_2 = Transaction {
        amount: 245,
        currency: String::from("USD"),
        email: String::from("leoul.mesfin.2017@gmail.com"),
        first_name: String::from("Leo"),
        last_name: String::from("Gezu"),
        tx_ref: String::from("rent5"),
    };

    chapa_rust::initialize_transaction(test_transaction_2);

    //chapa_rust::verify_transaction(String::from("test_transac_tx_ref"));
}
