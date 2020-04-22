mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Error;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static CPF_BLACKLIST: [&'static str; 10] = [
    "00000000000",
    "11111111111",
    "22222222222",
    "33333333333",
    "44444444444",
    "55555555555",
    "66666666666",
    "77777777777",
    "88888888888",
    "99999999999",
];

struct CPF {}

impl CPF {
    pub fn is_valid(cpf: &[u16]) -> bool {
        let digits = &cpf[..9];
        let checksum = CPF::compute_checksum(digits);

        cpf[9] == checksum[0] &&
        cpf[10] == checksum[1]
    }

    fn compute_checksum(cpf_digits: &[u16]) -> [u16; 2] {
        let first_digit = cpf_digits.iter()
            .enumerate()
            .map(|(idx, digit)| *digit * (10 - idx as u16))
            .sum::<u16>() * 10 % 11 % 10;

        let second_digit = (2 * first_digit + cpf_digits.iter()
            .enumerate()
            .map(|(idx, digit)| *digit * (11 - idx as u16))
            .sum::<u16>()) * 10 % 11 % 10;

        [first_digit, second_digit]
    }
}

#[wasm_bindgen]
pub fn isValid(cpf: String) -> Result<bool, JsValue> {
    if cpf.len() != 11 {
        return Err(Error::new("cpf must contain exactly 11 characters").into());
    }

    if !cpf.chars().all(|ch| ch.is_numeric()) {
        return Err(Error::new("cpf must contain only numeric characters").into());
    }

    if CPF_BLACKLIST.iter().any(|bad_cpf| bad_cpf == &cpf.as_str()) {
        return Ok(false)
    }

    let digits = cpf.chars().map(|c| c.to_digit(10).unwrap() as u16).collect::<Vec<_>>();
    Ok(CPF::is_valid(&digits[..]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn known_valid_cpf() {
        assert_eq!(
            isValid("41154981452".to_owned()).unwrap(),
            true
        )
    }

    #[test]
    fn off_by_one() {
        assert_eq!(
            isValid("41154981453".to_owned()).unwrap(),
            false
        )
    }

    #[test]
    fn bad_cpf() {
        assert_eq!(
            isValid("11111111111".to_owned()).unwrap(),
            false
        )
    }

    #[test]
    fn too_short() {
        assert_eq!(
            isValid("1234567890".to_owned()),
            Err(Error::new("cpf must contain exactly 11 characters").into())
        )
    }

    #[test]
    fn invalid_input() {
        assert_eq!(
            isValid("1234567890a".to_owned()),
            Err(Error::new("cpf must contain only numeric characters").into())
        )
    }
}