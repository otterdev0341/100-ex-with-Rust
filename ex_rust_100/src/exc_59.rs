/*
Write a  program that calculates and prints the sum of the first 100 non-negative integers (from 1 to 100).

Use a loop to find this sum.

🔹 Expected Output:
The expected output is the value of the sum:

5050

*/

fn sum_positive_hundrednum() {
    
    let mut result = 0;
    
    for x in 1..=100 {
        result = result + x;
    }
    println!("{}",result);
}

#[cfg(test)]
mod test {
    use super::sum_positive_hundrednum;


    #[test]
    fn test_1() {
        sum_positive_hundrednum();
    }
}