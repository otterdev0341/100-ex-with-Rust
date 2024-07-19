/*
Write a  program that calculates the factorial of a given number n.

The value of n should be entered by the user.

The program must print the result and use a loop to calculate it.

The factorial of a number n is denoted as n! and it is the result of multiplying all the positive integers that are less than or equal to n.

For example, 3! = 3 * 2 * 1.

0! is equal to 1.
*/


fn do_factorial(n: i32) {
    let mut result = 1;
    if n == 0 {
        println!("1");
        return;
    }
    for x in 1..=n {
        result *= x;
    }
    println!("{}",result);
}


#[cfg(test)]
mod test {
    use super::do_factorial;


    #[test]
    fn test_1() {
        do_factorial(10);
    }
}