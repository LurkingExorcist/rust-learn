fn main() {
    another_function(add(forty_two(), 1));
}

fn forty_two() -> i32 {
    42
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}