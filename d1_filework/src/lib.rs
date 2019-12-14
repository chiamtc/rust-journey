mod error;
pub use error::TransactionError;
use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}


//using Option , before using failure crate
/*pub fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    //ok() transform Result into Option. refers to docs
    let trans = get_transaction_type_error(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t)
        }
    }
    None
}*/

//using failure crate, converting Option to Result usage
pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    //ok() transform Result into Option. refers to docs
    let trans = get_transaction_type_error(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t)
        }
    }
    Err(TransactionError::Mess("could not find transaction with that name").into())
}

//verbose way
pub fn get_transaction(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string())
    };
    Ok(t)
}

//this func is using custom build transaction error instead of string
//e.into() is to get the specific type of error we implemented in enum TransactionError instead of e.to_string()
pub fn get_transaction_type_error(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.into())
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(t) => t,
        Err(e) => return Err(e.into())
    };
    Ok(t)
    //or a shorter version using ? operator, only applicable to Result
//    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?
}


//combinators
pub fn get_transaction_b(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}
