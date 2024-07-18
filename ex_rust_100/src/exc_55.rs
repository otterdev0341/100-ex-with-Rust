/*
Write a  program that simulates an interactive calculator with the basic arithmetic operations in  (addition, subtraction, multiplication, division, integer division, and modulo).

The program must interact with the user by asking for the values and the type of operation that will be performed.

The output should include the values, the operation, and the result (as shown below).

The type of operation will be denoted by an integer.

- 1 for addition

- 2 for subtraction

- 3 for multiplication

- 4 for division

- 5 for integer division

- 6 for modulo

The program should present an initial message to the user describing the types of operations and the integer that has to be entered to select each one of them.

If the values entered by the user are invalid for the operation selected, a descriptive message should be displayed. For example, for a division operation the denominator cannot be 0.

If the user enters an invalid integer to select the operation, print

"Please choose a valid operation"

    {a,b}       {operation}         {output}
    1,2             1               The result of 1 + 2 is 3
    2,3             2               the result of 2 - 3 is -1
    3,4             3               the result of 3 * 4 is 12
    5,5             4               the result of 5/5 = 1.0
    15,2            2               the result of 15 // 2 = 7
    16,3            6               the result of 16 % 3 is 1

    For example:

=== Welcome to your Interactive Calculator ===
Please enter the first value: 16
Please enter the second value: 3
Great! Now enter the operation.
These are the available options:
1 - Addition
2 - Subtraction
3 - Multiplication
4 - Division
5 - Integer Division
6 - Modulo
--> Enter the corresponding integer: 6
The result of 16 % 3 is: 1

*/


use std::io;


pub fn mini_cal() {
    println!("=== Welcome to your Interactive Calculator ===");
    println!("Please enter the first value:");
    let first_value = match take_number() {
        Some(number) => number,
        None => 0,
    };
    println!("Please enter second value:");
    let second_value = match take_number() {
        Some(number) => number,
        None => 0,
    };
    println!("Great! Now enter the operation.");
    println!("These are the available options:
    1 - Addition
    2 - Subtraction
    3 - Multiplication
    4 - Division
    5 - Integer Division
    6 - Modulo");
    println!("--> Enter the corresponding integer:");
    let operation = match take_number() {
        Some(operation) => operation,
        None => 0,
    };
    match operation {
        1 => println!("The result of {} + {} is {}",first_value, second_value, first_value + 2),
        2 => println!("The result of {} - {} is {}",first_value, second_value, first_value - second_value),
        3 => println!("The result of {} * {} is {}",first_value, second_value, first_value * second_value),
        4 => println!("The result of {} / {} = {}",first_value, second_value, first_value / second_value),
        5 => println!("The result of {} // {} = {}",first_value, second_value,first_value / second_value),
        6 => println!("The result of {} % {} is {}",first_value, second_value, first_value % second_value),
        _ => println!("Please choose a valid operation"),
    }
}

pub fn take_number() -> Option<i32> {
    
    
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("invalid data please try again");
    }
    let input = buffer.trim().to_owned();
    
    let parsed_input: Result<i32,_> = input.parse();
    match parsed_input {
        Ok(number) => return Some(number),
        Err(_) => None,
    }

}

#[cfg(test)]
mod test {
    use super::mini_cal;

    
    #[test]
    fn test_1() {
        mini_cal();
    }
}