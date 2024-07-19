/*
Write a  program that prints the integers between a given number n and 1 (in descending order, both inclusive).

The value of n should be entered by the user and you may assume that it is an integer greater than or equal to 1.

Use a loop to print each number on a separate line.

Expected Output:
If n is equal to 6:

6
5
4
3
2
1
If n is equal to 10:

10
9
8
7
6
5
4
3
2
1

*/

fn print_range_reverse(n: i32) {
    
    for x in (1..=n).rev() {
        println!("{}",x);
    }
}

#[cfg(test)]
mod test {
    use super::print_range_reverse;


    #[test]
    fn test_1() {
        print_range_reverse(6);
    }
}