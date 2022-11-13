fn main() {

    // Primitive Types
    //
    // Scalar Types
    // Integer = 5
    // Floating Point = 5.5
    // Bool = true or false
    // Char = a or 一 or 😊
    //
    // Compound Types
    // Tuple = (5, 5.5, true, 'a')
    // Array = [1, 2, 3, 4, 5]

    // Integers
    // +------+--------+----------+
    // | bits | signed | unsigned |
    // |------|--------|----------|
    // | 8    | i8     | u8       |
    // | 16   | i16    | u16      |
    // | 32   | i32    | u32      |
    // | 64   | i64    | u64      |
    // | 128  | i128   | u128     |
    // | arch | isize  | usize    |
    // +------+--------+----------+
    //
    // Signed
    // i8 = -128 to 127
    //
    // Unsigned
    // u8 = 0 to 255

    // let x: i8 = 5; // default i32
    // let x: u8 = 5;
    // let x = 5_u8;

    // let y: u8 = x - 20; // overflow
    // let y: i8 = x * 200;

    // let x = 5;
    // let y = 123_456_789_0;
    // let h = 0xff;
    // let o = 0o77;
    // let b = 0b1101_0001;
    // let by = b'A';

    // let x: f32 = 40.1; // default f64

    // let x = true; // 1
    // let y = false; // 0

    // let letter = 'a'; // char
    // let letter = '😊';

    // Tuples
    // let mut numbers = (1, 2, 3);
    // let (a, _b, _c) = numbers;
    // println!("{:?}", numbers);
    // println!("{:?}", numbers.0);
    // println!("{:?}", a);
    // numbers.0 = 50;
    // println!("{:?}", numbers.0);

    // Arrays
    // let x = [1, 2.2, true, 'a'];
    // let x = [1, 2, 3];
    // println!("{:?}", x[0]);
    // println!("{:?}", x[10]); // out of bound
    // println!("{:?}", &x[1..]);

}
