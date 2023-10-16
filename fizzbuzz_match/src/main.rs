
fn fizzbuzz(i: usize) -> String {
    let remainder_tuple = (i % 3, i % 5);
    match remainder_tuple {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => i.to_string(),
    }
}

fn main() {
    for i in 1..=100 {
        println!("fizzbuzz for {} is {}", i, fizzbuzz(i));
    }
}