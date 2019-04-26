
extern crate jsonwebtoken as jwt;

use serde::{Deserialize, Serialize};
use jwt::{encode, decode, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use jwt::TokenData;

static mut SECRET: Option<String> = None;
static TIME: u64 = 15000;

#[derive(Serialize,Deserialize)]
pub struct Token {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64
}

pub fn init(secret: String) {
    unsafe {
        SECRET = Some(secret)
    }
}

pub fn create(id: i64) -> Token {
    trace!("Creating token...");
    let jwt = Claims {
        sub: id.to_string(),
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + TIME
    };
    Token {
        token: to_token(&jwt)
    }
}

pub fn renew(claims: &mut Claims) -> Token {
    trace!("Renewing token...");
    claims.exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + TIME;
    Token {
        token: to_token(&claims)
    }
}

pub fn from_token(token: &String) -> Result<TokenData<Claims>, jwt::errors::Error> {
    trace!("Parsing token...");
    unsafe {
        decode::<Claims>(token, SECRET.as_ref().unwrap().as_ref(), &Validation::default())
    }
}

fn to_token(my_claims: &Claims) -> String {
    trace!("Serializing token...");
    unsafe {
        encode(&Header::default(), my_claims, SECRET.as_ref().unwrap().as_ref()).unwrap()
    }
}

