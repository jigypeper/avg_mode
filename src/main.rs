use std::io;
use std::convert::TryFrom;

#[derive(Debug)]
struct Mode {
    number: u32,
    count: u32,
}

fn main() {
    let mut list = Vec::new();

    println!("Average and Mode Calculator!!");
    
    // TODO: get rid if loop, just parse one string of number, split on white space

    loop {

        let mut value: String = String::new();

        println!("Enter a number: ");

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read message");

        let value: u32 = match value.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => break,
        };

        list.push(value);
        
    }
    list.sort();

    let mut total = 0;
    let length = u32::try_from(list.len()).unwrap();

    for number in &list {
        total += number;
        println!("{}", number);
    }

    let average = total as f32 / length as f32;
    println!("Total is {}", total);
    println!("Length is {}", length);
    println!("Average is {}", average);

    let mut mode = Mode{number: 0, count: 0};
    
    for number in &list {
        let count = count_eq(&list, *number);
        if count > mode.count {
            mode.number = *number;
            mode.count = count;
        }
    }

    
    if mode.count > 1 {
        println!("Mode is {} apearing {} times", mode.number, mode.count);
    } else {
        print!("There is no mode");
    }
    
    
    
}

fn count_eq(vec: &Vec<u32>, num: u32) -> u32 {
    let mut counter = 0;
    for i in vec {
        if *i == num {
            counter += 1;
        }
    }
    counter
}
