/*
    Write a Python program that prints the smallest of three integers a, b, and c.

    {input}         {output}
    1,2,3           1
    2,2,2           2
    -1,-3,-4        -4

*/
fn min_of_three(a: i32, b: i32, c: i32) {
   
    let min_value = if a < b {
        if a < c { a } else { c }
    } else {
        if b < c { b } else { c }
    };
   println!("{}",min_value);
}


#[cfg(test)]
mod test {
    use super::min_of_three;


    #[test]
    fn test_1() {
        let a = 1;
        let b = 2;
        let c = 3;
        min_of_three(a, b, c);
    }

    #[test]
    fn test_2() {
        let a = 2;
        let b = 2;
        let c = 2;
        min_of_three(a, b, c);
    }

    #[test]
    fn test_3() {
        let a = -1;
        let b = -3;
        let c = -4;
        min_of_three(a, b, c);
    }
}