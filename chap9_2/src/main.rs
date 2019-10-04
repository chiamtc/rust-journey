use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main() {
    let f = File::open("example.txt");
   /* let f = match f{
        Ok(file) => file,
        Err(error) =>{
            panic!("Problem opening the file: {:?}", error)
        }
    };*/
   /* let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=> panic!("Problem creating the file: {:?}", e),
            },
            other_error=> panic!("Problem opening the file: {:?}", other_error)
        }
    };*/
    //veteran level
    let f = File::open("example.txt").unwrap_or_else(|error|{
       if error.kind() == ErrorKind::NotFound{
           File::create("hello.txt").unwrap_or_else(|error|{
               panic!("Problem creating the file: {:?} ", error);
           })
       } else{
           panic!("Problem opening the file: {:?} ", error);
       }
    });


    let f2 = File::open("example.txt").unwrap();
    //using unwrap() directly after open =
    /*
     is a shortcut method that is implemented just like the match expression we wrote in Listing 9-4.
     If the Result value is the Ok variant, unwrap will return the value inside the Ok.
     If the Result is the Err variant, unwrap will call the panic! macro for us
     */
    println!("Hello, world!");


    let f3 = File::open("test.txt").expect("Failed ot open test.txt file");

}

fn read_username_from_file() -> Result<String,io::Error>{
    let f = File::open("test.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(e)=> return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

//source: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
fn read_username_from_file2() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    //even shorter form
    /*let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s);
    */
}
