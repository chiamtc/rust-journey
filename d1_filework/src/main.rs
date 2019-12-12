extern crate d1_filework;
use dl_filework::*;
use serde_derive::*;

//we implemented "From<&'static str>" for main() to return Result<(),TransactionError>
fn main() -> Result<(), TransactionError>{
    println!("Hello, world!");
/*

    let trans = get_transaction_b("test_data/transactions.json").expect("Could not load transactions");
    for t in trans {
        println!("{:?}", t);
    }
*/

    //ok_or() convert option to Result by providing an error if option is none.
    let t = get_first_transaction_for("test_data/transactions.json", "Matt").ok_or("could not get first transaction")?;
    println!("Matt's first transaction : {:?}", t);
    Ok(())
}
