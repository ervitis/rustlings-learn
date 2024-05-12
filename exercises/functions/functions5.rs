// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
    println!("The square of 4.5 is {}", square_2(4.5))
}

fn square(num: i32) -> i32 {
    return num * num;
}

fn square_2(num: f32) -> f32 {
  num * num
}