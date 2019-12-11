use serde_derive::*;

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Mess(&'static str)
}

impl From<std::io::Error> for TransactionError {
    //from() is a requried function to implement from From trait
    // signature -> from(T) -> Self
    // Self is a reserved word, the type we're working with right now aka TransactionError since the impl is "for TransactionError"
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    //from() is a requried function to implement from From trait
    // signature -> from(T) -> Self
    // Self is a reserved word, the type we're working with right now aka TransactionError since the impl is "for TransactionError"
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError{
    fn from(e:&'static str ) -> Self{
        TransactionError::Mess(e)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() -> Result<(), TransactionError>{
    println!("Hello, world!");
/*

    let trans = get_transaction_b("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
*/

    let t = get_first_transaction_for("test_data/transactions.json", "Matt").ok_or("could not get first transaction")?;
    println!("Matt's first transaction : {:?}", t);
    Ok(())
}

fn get_first_transaction_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transaction_type_error(fname).ok()?;
    for t in trans {
        if t.from == uname {
            return Some(t)
        }
    }
    None
}

fn get_transaction(fname: &str) -> Result<Vec<Transaction>, String> {
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
fn get_transaction_type_error(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
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

fn get_transaction_b(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}