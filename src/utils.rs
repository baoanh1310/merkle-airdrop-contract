use crate::*;

pub fn assert_self() {
    assert_eq!(env::predecessor_account_id(), env::current_account_id());
}

pub fn yton(yocto_amount: Balance) -> Balance {
    (yocto_amount + (5 * 10u128.pow(23))) / 10u128.pow(24)
}

pub fn balance_to_str(amount: Balance) -> String {
    let result = amount.to_string();
    // println!("{:?}", result);
    result
}

pub fn get_string_from_hash(buf: Vec<u8>) -> String {
    let s = String::from_utf8_lossy(&buf);
    s.to_string()
}
