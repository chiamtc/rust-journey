use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction{
    from : String,
    to : String,
    amount: u64
}

fn main() {
    println!("Hello, world!");

    let trans = get_transaction_b("test_data/transactions.json").expect("Could not load transactions");
    for t in trans{
        println!("{:?}", t);
    }
}

fn get_transaction(fname: &str) -> Result<Vec<Transaction>, String>{
//    Err("No trans".to_string());
    let s = match std::fs::read_to_string(fname){
        Ok(v)=> v,
        Err(e) => return Err(e.to_string())
    };
    let t:Vec<Transaction> = match serde_json::from_str(&s){
        Ok(t) => t,
        Err(e)=> return Err(e.to_string())
    };
    Ok(t)
//    Ok(Vec::new())
}

fn get_transaction_b(fname: &str) -> Result<Vec<Transaction>, String>{
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}