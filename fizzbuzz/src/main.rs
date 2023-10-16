
fn fizzbuzz(i: usize) -> String {
    let mut string: String;
    if i % 3 == 0 {
        string = String::from("Fizz");
    } else {
        string = String::new();
    }

    if i % 5 == 0 {
        string = format!("{}Buzz", string);
    }

    if string.is_empty() {
        i.to_string()
    } else {
        string
    }
}

fn main() {
    for i in 1..=100 {
        println!("fizzbuzz for {} is {}", i, fizzbuzz(i));
    }
}
