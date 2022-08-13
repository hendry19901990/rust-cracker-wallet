#[warn(unused_must_use)]
extern crate num_bigint;
extern crate web3;

mod eth_wallet;
mod db;
mod rich_list;

use std::thread;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

const N_THREADS: usize = 11;

fn main() {
    read_by_range_async_all();
}

fn read_by_range_async_all(){
    let mut children = vec![];

    let list: HashSet<String> = rich_list::get_list();
    let lock = Arc::new(RwLock::new(list));

    let rang_list: Vec<(String, String)> = vec![ 
        ("115792089237316195423570985008687906843269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687906853269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687906853269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687906953269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687906953269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907053269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907053269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907153269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907153269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907253269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907253269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907353269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907353269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907453269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907453269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907553269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907553269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907653269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907653269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907753269984665640564039457584007913129639936".to_owned()),
        ("115792089237316195423570985008687907753269984665640564039457584007913131063885".to_owned(), "115792089237316195423570985008687907853269984665640564039457584007913129639936".to_owned()) // 111251222992715560308921142549645508283961922051594736365803047501698180327189
    ];

    let rang_list_lock = Arc::new(RwLock::new(rang_list));
    for i in 0..N_THREADS {

        let c_lock = Arc::clone(&lock);
        let rang_c_lock = Arc::clone(&rang_list_lock);

        children.push(thread::spawn(move || {
            if let Ok(r_rang) = rang_c_lock.read(){
                let tup = &r_rang[i];
                let _str  = &tup.0;
                let _str_end  = &tup.1;

                let mut big_int = eth_wallet::str_to_big(_str.to_string(), 10u32);
                let big_int_end = eth_wallet::str_to_big(_str_end.to_string(), 10u32);

                if let Ok(r) = c_lock.read(){
                        while big_int < big_int_end { 
                            println!("{} => {:}", &i, big_int.to_string());
            
                            call(&big_int, &r);
                            big_int = big_int * 616u64;
                        }
                }

                }
           
       }));
   }
   for child in children {
       // Wait for the thread to finish. Returns a result.
       let _ = child.join();
   }
}

fn call(big_int: &num_bigint::BigUint, list:  &HashSet< String> ) {
    if let Some(wallet) = eth_wallet::generate_keypair(big_int) {
        if db::exist_address(list, &wallet.public_address) {
            if let Ok(_) = db::store(wallet.secret_key.clone(), wallet.public_address.clone()){
                println!("Secret Key: {}", &wallet.secret_key.to_string());
                println!("Address: {}", &wallet.public_address.to_string()); 
            }
        }
    } else {
        println!("Errors!!!")
    }
    
}

