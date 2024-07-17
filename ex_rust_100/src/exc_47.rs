/*
    Write a  program that prints the maximum of three integers (a, b, c).

    {a,b,c}             {output}
    {1,3,4}             4
    {1,4,15}            15
    {3,3,3}             3

    */

fn max_of_three(a: i32, b: i32, c: i32) {
    if a > c {
        if b < c {
            println!("{}",a);
        } else {
            println!("{}",b);
        }
    } else {
        println!("{}",c);
    }
}


#[cfg(test)]
mod test {
    use super::max_of_three;


    #[test]
    fn test_1() {
        let a = 1;
        let b = 3;
        let c = 4;
        max_of_three(a, b, c);
    }

    #[test]
    fn test_2() {
        let a = 1;
        let b = 4;
        let c = 15;
        max_of_three(a, b, c);
    }

    #[test]
    fn test_3() {
        let a = 3;
        let b = 3;
        let c = 3;
        max_of_three(a, b, c);
    }
}