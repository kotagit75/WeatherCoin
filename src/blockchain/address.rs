use regex::Regex;

use crate::util::key::PK;

pub type Address = PK;

pub fn is_valid_address(address: &Address) -> bool {
    let Ok(re) = Regex::new(r"^[a-fA-F0-9]+$") else {
        return false;
    };
    address.der.starts_with("30") && re.is_match(&address.der)
}
