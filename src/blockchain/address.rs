use regex::Regex;
use std::sync::LazyLock;

use crate::util::key::PK;

pub type Address = PK;

static ADDRESS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-fA-F0-9]+$").unwrap());

pub fn is_valid_address(address: &Address) -> bool {
    address.der.starts_with("30") && ADDRESS_RE.is_match(&address.der)
}
