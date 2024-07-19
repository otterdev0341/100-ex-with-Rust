/*
Write a  program that prints a triangular pattern like the ones shown below in the examples.

Each row must have its corresponding number of asterisks. The first row contains one asterisk. The second row contains two asterisks. The third row contains three asterisks and so on.

The number of rows should be determined by the value of n, a value entered by the user.

🔹 Expected Output:
If the value of n is 3:

* 
* * 
* * * 
If the value of n is 5:

* 
* * 
* * * 
* * * * 
* * * * * 
 */


fn print_star_1(n: i32) {
    let mut i = 1;
    for x in 1..=n {

        while i <= x {
            print!("*");
            print!(" ");
            i = i + 1;
        }
        i = 1;
        println!();

    }
}

#[cfg(test)]
mod test {
    use super::print_star_1;


    #[test]
    fn test_1() {
        print_star_1(5);
    }
}

