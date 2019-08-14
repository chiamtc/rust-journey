fn main() {
    //integer types
    /*
     Length 	Signed 	Unsigned
        8-bit 	i8 	u8
        16-bit 	i16 	u16
        32-bit 	i32 	u32
        64-bit 	i64 	u64
        128-bit 	i128 	u128
        arch 	isize 	usize

        signed = iX = +/-
        unsigned = uX = + only

        Number literals 	Example
        Decimal 	98_222
        Hex 	0xff
        Octal 	0o77
        Binary 	0b1111_0000
        Byte (u8 only) 	b'A'

        NOTE: good read = Integer overflow in development and production mode
        https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
    */

    let num1: i32 = -5;
    let num2: u32 = 5;
    let float1: f32 = 3.2;
    let float2: f64 = 3.1;
    let hex_number: u32 = 0x40;
    let bool_test: bool = true;
    let c = 'z';

    //TUple
    let tup: (i32, f64, u8) = (-500, 3.2, 5);
    let (a, b, c) = tup; //destructuring
    println!("{} , {} ,{}, {} ", a, b, c, tup.2);

    let array_type: [i32; 4] = [1, 2, 3, 4]; //create new array with i32 type and only has 4 elements
    let a = [3;5]; // [3,3,3,3,3];  specifies initial value of '3' in lenght of 5 of array
    println!(a[0]) //to access array

    println!("Hello, world!, {}, {},{}, {},{}", hex_number, num1, num2, float1, float2);
}
