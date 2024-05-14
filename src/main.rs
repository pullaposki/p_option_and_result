fn take_fifth(value:Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option:Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(value) => println!("The fifth element is {}", value),
            None => println!("There is no fifth element"),
        }
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok (number),
        _ => Err("The number is not five".to_string()),
    }
}

fn main() {
    let new_vec = vec![2,4];
    let bigger_vec = vec![2,4,6,8,10,12];
    let mut option_vec = Vec::new(); // Vec to hold Option<i32> values

    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger_vec));

    handle_option(option_vec);

    let mut result_vec = Vec::new();

    for number in 2..7 {
        result_vec.push(check_if_five(number));
    }
    println!("{:?}", result_vec);
}
