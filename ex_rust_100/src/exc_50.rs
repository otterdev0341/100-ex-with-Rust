/*
Write a  program that prints "Equal" if three numbers a, b, and c are equal.

If at least one number if different, the program should print "Not Equal".

    {input}             {output}
    3,3,3               Equal
    3,4,3               Not Equal
    3,4,4               Not Equal
    1,2,3               Not Equal
*/


fn equal_in_three(a: i32, b: i32, c: i32) {
    
    let mut result = "Equal".to_owned();

    if a != b || b != c || c != a {
        result = "Not Equal".to_owned();
    }

    println!("{}",result);
}



#[cfg(test)]
mod test {
    use super::equal_in_three;

    #[test]
    fn test_1() {
        let a = 3;
        let b = 3;
        let c = 3;
        equal_in_three(a, b, c);
    }

    #[test]
    fn test_2() {
        let a = 3;
        let b = 4;
        let c = 3;
        equal_in_three(a, b, c);
    }

    #[test]
    fn test_3() {
        let a = 3;
        let b = 4;
        let c = 4;
        equal_in_three(a, b, c);
    }

    #[test]
    fn test_4() {
        let a = 1;
        let b = 2;
        let c = 3;
        equal_in_three(a, b, c);
    }
}