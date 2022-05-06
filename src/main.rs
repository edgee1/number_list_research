use std::io;
use std::collections::HashMap;



fn main() {
    println!("Введите ряд чисел через пробел: ");

    let mut number_row = String::new();

    io::stdin()
        .read_line(&mut number_row)
        .expect("Failed to read line");

    let mut number_vector: Vec<i32> = Vec::new();

    for number in number_row.split_whitespace() {
        number_vector.push(number.parse().expect("Введите числа"));
    }

    number_vector.sort();
    let index: usize = (number_vector.len() / 2) as usize;

    if number_vector.len() % 2 != 0 {
        println!("Median is {}", number_vector[index]);
    }else {
        println!("Median is {}, {}", number_vector[index - 1], number_vector[index]);
    }

    println!("range is {}", number_vector[number_vector.len() - 1] - number_vector[0]);

    let mut hash_map_values = HashMap::new();
    let mut max_value = 0;
    for i in number_vector{
        let count = hash_map_values.entry(i).or_insert(0);
        *count += 1;
        if *count > max_value {max_value = *count};
    }
    println!("Mode: ");
    for (key, value) in hash_map_values {
        if value == max_value {
            println!("{}", key);
        }
    }
}
