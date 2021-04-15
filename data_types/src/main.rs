fn main() {
    // Числовые типы данных
    /*
        Размер	Знаковый	Беззнаковый
        8-bit	i8	        u8
        16 бит	i16	        u16
        32 бита	i32	        u32
        64 бита	i64	        u64
        128 бит	i128	    u128
        arch	isize	    usize
    */

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    
    // Логические типы данных
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    // Символьный тип даных
    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Строковый тип данных
    let hello: &str = "world";

    // Кортежи
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {}", y);

    // Массивы
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
}
