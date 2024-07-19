/*

Write a  program that prints the first 15 positive integers (starting from 1).

Print the numbers one per line using a loop and the range() function.
    Expected Output:
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
*/

fn print_positive_to_fifteen() {
    
    for x in 1..=15 {
        println!("{}",x);
    }
}

#[cfg(test)]
mod test {
    use super::print_positive_to_fifteen;


    #[test]
    fn test_1() {
        print_positive_to_fifteen();
    }
}