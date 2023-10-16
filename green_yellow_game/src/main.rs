use std::collections::HashMap;
use std::io;

fn decrement_if_exist(counter: &mut HashMap<&u8, u8>, num: &u8) -> bool {
    match counter.get_mut(num) {
        Some(value) => {
            if *value == 0 {
                return false;
            }
            *value -= 1;
        }
        None => return false,
    }

    true
}


fn calc_green_and_yellow(guess: &[u8; 4], secret: &[u8; 4]) -> String {
    let mut result_string = String::new();
    let mut secret_counter = HashMap::new();

    secret
        .iter()
        .for_each(|num| {
            secret_counter.entry(num).and_modify(|num| *num += 1).or_insert(1);
        });

    guess
        .iter()
        .enumerate()
        .for_each(|(i, num)| {
            if num == &secret[i] {
                result_string.push_str("🟩");
                decrement_if_exist(&mut secret_counter, &num);
            } else if decrement_if_exist(&mut secret_counter, &num) {
                result_string.push_str("🟨");
            } else {
                result_string.push_str("⬜");
                return;
            }
        });

    result_string
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret: [u8; 4] = rand::random();
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)?;

    let guess: [u8; 4] = user_input.trim().split(' ')
        .map(|num_str| num_str.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    println!("Correct 🟩, Swapped 🟨, Wrong ⬜ Guesses: \n{:?}", 
        calc_green_and_yellow(&guess, &secret)
    );

    Ok(())
}

#[test]
fn all_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[5, 6, 7, 8], &[1, 2, 3, 4]),
        "⬜⬜⬜⬜"
    );
}

#[test]
fn all_green() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩🟩🟩🟩"
    );
}

#[test]
fn one_wrong() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn all_yellow() {
    assert_eq!(
        &calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨🟨🟨🟨"
    );
}

#[test]
fn one_wrong_but_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩🟩🟩⬜"
    );
}

#[test]
fn one_right_others_duplicate() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩⬜⬜⬜"
    );
}

#[test]
fn two_right_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨🟩🟩🟨"
    );
}

#[test]
fn two_wrong_two_swapped() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨⬜⬜🟨"
    );
}

#[test]
fn a_bit_of_everything() {
    assert_eq!(
        &calc_green_and_yellow(&[1, 9, 4, 3], &[1, 2, 3, 4]),
        "🟩⬜🟨🟨"
    );
}