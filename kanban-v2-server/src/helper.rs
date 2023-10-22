use pwhash::sha512_crypt;
use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z]{2,}$").unwrap();
    regex.is_match(email)
}

pub fn hash_pw(password: &str) -> String {
    sha512_crypt::hash(password).unwrap()
}

pub fn verify_pw(req_password: &str, db_password: &str) -> bool {
    sha512_crypt::verify(req_password, db_password)
}
