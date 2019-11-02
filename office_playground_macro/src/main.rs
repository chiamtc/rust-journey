use std::collections::HashMap;
macro_rules! a_macro {
    () => {
        println!("this is from macro")
    }
}

//A variadic interface
//allows to have numerous params

macro_rules! x_and_y {
    (x=> $e:expr) => (println!("X:{} ", $e));
    (y=> $e:expr) => (println!("Y:{}", $e));

}

macro_rules! build_fn {
//identifier = just functiona name or variable name
    ($func_name:ident) =>(
        fn $func_name(){
            println!("You called {:?}()", stringify!($func_name))
        }
    )
}

macro_rules! print_ex {
    ($e:expr) => (
    println!("{:?} = {:?}", stringify!($e), $e)
    )
}

macro_rules! exame {
    ($l:expr; and $r:expr)=>(
        println!("{:?} and {:?} is {:?}", stringify!($l), stringify!($r), $l && $r);
    );

    ($l:expr; or $r:expr)=>(
        println!("{:?} or {:?} is {:?}", stringify!($l), stringify!($r), $l || $r);
    );
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start:expr; $end:expr] , $cond:expr)=>{
{let mut vec = Vec::new();
        for num in $start..$end +1{
            if $cond(num){
                vec.push(num)
            }
        }
        vec
}
    };
}

fn even(x: i32) -> bool {
    x % 2 == 0
}

fn odd(x: i32) -> bool {
    x % 2 != 0
}

macro_rules! calc{
(eval $e:expr) => {{
    {
    let val:usize = $e;
    println!("{} = {} ", stringify!{$e}, val)
    }
}};

(eval $e:expr, $(eval $es:expr),*) =>{
{
calc!{eval $e}
calc!{ $(eval $es), +}
}
}
}

macro_rules! new_map {
//($ (....)* ) = this expression can be repeated to infinite
//you can remove the comma "," , it's just for syntax purpose,to look good
//replacing * with + means that this macro requires minimum one value to be consumed
//* = can writes without any expression
//+ = write the expression with minimum one value
($($key:expr => $val:expr),*)=>{
    {
        let mut map = HashMap::new();
        $(
        map.insert($key, $val);
        )*
        map
    }
}
}


fn main() {
    println!("Hello, world!");
    a_macro!();

    x_and_y!(x=>10);

    build_fn!(Hi_there);
    Hi_there();

    print_ex!({
    let y = 20;
    let z = 30;
    z+y +10 * 3 *100
    });

    exame!(1 == 1; and 3 == 2+1);

    let evens = compr!(x|x <- [1;10],even);
    println!("{:?}", evens);

    let m = new_map!(
    "one" =>1,
    "two" => 2,
    "three"=> 3
    );

    calc!{
    eval 4*5,
    eval 4+10
    }
}

