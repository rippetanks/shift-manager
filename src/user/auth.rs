
extern crate crypto;
extern crate jwt;

use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use ring::{rand, pbkdf2, digest};
use ring::rand::SecureRandom;
use data_encoding::HEXUPPER;
use crypto::sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use jwt::{Header, Registered, Token};

use crate::controller::Extras;
use crate::user::model::User;
use crate::database::ShiftManagerDB;

pub struct ApiKey {
    pub sub: i64,
    pub exp: u64
}

pub struct AuthInfo {
    pub stored_key: String,
    pub salt: String
}

const ITERATION: u32 = 16;
const SALT_LEN: usize = 8;
const PWD_LEN: usize = digest::SHA256_OUTPUT_LEN;

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
    Broken
}

///
/// Create Auth info.
pub fn create_auth(password: &str) -> Result<AuthInfo, ()> {
    let rng = rand::SystemRandom::new();
    // salt
    let mut salt = [0u8; SALT_LEN];
    rng.fill(&mut salt).unwrap();
    // salted password
    let mut salted_pwd = [0u8; PWD_LEN];
    pbkdf2::derive(&digest::SHA256, ITERATION, &salt, (*password).as_bytes(), &mut salted_pwd);
    // convert to string salted password and salt
    let salt_str = HEXUPPER.encode(&salt);
    trace!("salt_str: {}", salt_str);
    let stored_key = HEXUPPER.encode(&salted_pwd);
    trace!("stored_key: {}", stored_key);
    Ok(AuthInfo {
        stored_key,
        salt: salt_str
    })
}

///
/// Create the token for the current user session.
pub fn create_token(user: &User, extra: &State<Extras>) -> Result<String, Status> {
    trace!("extras: {:?}", extra);
    let header: Header = Default::default();
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + extra.jwt_exp;
    let claims = Registered {
        sub: Some(user.id.to_string()),
        exp: Some(exp),
        ..Default::default()
    };
    let token = Token::new(header, claims);
    token.signed(extra.jwt_key.as_bytes(), Sha256::new())
        .map(|token| token)
        .map_err(|e| {
            error!("Can not generate token caused by {:?}", e);
            Status::InternalServerError
        })
}

///
/// Check user login information
#[allow(unused_must_use)]
pub fn login(user: &User, pwd: &str, conn: &ShiftManagerDB) -> bool {
    let salt = HEXUPPER.decode(user.salt.as_bytes()).unwrap();
    let db_pwd = HEXUPPER.decode(user.pwd.as_bytes()).unwrap();
    let result = pbkdf2::verify(&digest::SHA256, ITERATION,
                                salt.as_slice(),
                                (*pwd).as_bytes(),
                                db_pwd.as_slice());
    result.map(|_| {
        debug!("update last login for user {}", user.id);
        if !User::update_last_login(user.id, conn).is_ok() {
            error!("Can not update last login for user {}", user.id);
        }
    });
    result.is_ok()
}

// #################################################################################################

fn read_token(key: &str, secret: &String) -> Result<ApiKey, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|e| {
            error!("can not parse key {:?}", e);
            "Unable to parse key".to_string()
        })?;
    // verify token
    if token.verify(secret.as_bytes(), Sha256::new()) {
        Ok(ApiKey {
            sub: token.claims.sub.ok_or("sub not valid".to_string())?.parse::<i64>().unwrap(),
            exp: token.claims.exp.ok_or("exp not valid".to_string())?
        })
    } else {
        error!("token invalid {:?}", token);
        Err("Token not valid".to_string())
    }
}

fn is_token_valid(key: &ApiKey) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    key.exp > now
}

// #################################################################################################

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        let extra = request.guard::<State<Extras>>().unwrap();
        match keys.len() {
            0 => {
                warn!("Access denied! Missing API KEY.");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing))
            },
            1 => match read_token(keys[0], &extra.jwt_key) {
                Ok(api_key) if is_token_valid(&api_key) => {
                    debug!("ApiKey is valid!");
                    Outcome::Success(api_key)
                },
                Ok(_) => {
                    warn!("Access denied! Expired API KEY.");
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
                Err(_) => {
                    warn!("Access denied! Invalid API KEY.");
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
            },
            _ => {
                warn!("Access denied! Too much API KEY.");
                Outcome::Failure((Status::Unauthorized, ApiKeyError::BadCount))
            }
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        let conn = request.guard::<ShiftManagerDB>().unwrap();
        let key_outcome = request.guard::<ApiKey>();
        if key_outcome.is_failure() {
            // forward failure from ApiKey handler
            return Outcome::Failure(key_outcome.failed().unwrap());
        }
        let key = key_outcome.unwrap();
        let user = User::read_by_id(key.sub, &conn);
        match user {
            Ok(user) => {
                debug!("Access granted to user {}", user.id);
                Outcome::Success(user)
            },
            Err(e) => {
                warn!("Access denied to user {} caused by {}", key.sub, e);
                Outcome::Failure((Status::Unauthorized, ApiKeyError::Broken))
            }
        }
    }
}