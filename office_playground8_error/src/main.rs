use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn exit(x: Option<i32>) {
    match x {
        Some(0) => panic!("we got a 0"),
        Some(x) => println!("we're fine"),
        None => println!("we got nothing")
    }
}

fn main() {
//    exit(Some(1));
//    exit(Some(0));
    /* let f = match f {
         Ok(file) => file,
         Err(e) => {
             panic!("{}", e)
         }
     };*/


    let f = File::open("text.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("text.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "we couldnt create a new file {:?}",
                        e
                    )
                },
            }
        },
        Err(error)=>{
            panic!("couldnt open the file {:?}", error)
        }
    };
    //unwrap the actual data from Result or Option and output the data
    let f1 = File::open("text.txt").unwrap();

    let f2 = File::open("text.txt").expect("Could not handle the file");


}

fn read_file () -> Result<String, io::Error>{
   /* this is pretty verbose
    let f = File::open("text.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }*/

  /*
    using ? operator which ONLY applicable to Result<>
    let mut f = File::open("text.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)*/

    //to go even simpler

    let mut s = String::new();
    let mut f = File::open("text.txt")?.read_to_string(&mut s);
    Ok(s)
}