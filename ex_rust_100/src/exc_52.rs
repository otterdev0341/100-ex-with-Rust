/*
Write a  program that determines if three numbers (a, b, and c) are in increasing order, decreasing order, or none.

If a > b > c, print "Decreasing Order".

If a < b < c, print "Increasing Order".

Else, print "None".

    {a,b,c}         {output}
    1,2,3           "increasing order"
    3,2,1           "decreasing order"
    1,1,1           "None"
    1,2,1           "None"

*/

fn order_of_three(a: i32, b: i32, c: i32) {
    if a > b && b > c {
        println!("decreasing order");
    } else if a < b && b < c {
        println!("Increasing oder");
    } else {
        println!("None");
    }
}

#[cfg(test)]
mod test {
    use super::order_of_three;


    #[test]
    fn test_1() {
        //1,2,3           "increasing order"
        let a = 1;
        let b = 2;
        let c = 3;
        order_of_three(a, b, c);
    }

    #[test]
    fn test_2() {
        //3,2,1           "decreasing order"
        let a = 3;
        let b = 2;
        let c = 1;
        order_of_three(a, b, c);
    }

    #[test]
    fn test_3() {
        //1,1,1           "None"
        let a = 1;
        let b = 1;
        let c = 1;
        order_of_three(a, b, c);
    }

    #[test]
    fn test_4() {
        //1,1,1           "None"
        let a = 1;
        let b = 2;
        let c = 1;
        order_of_three(a, b, c);
    }

}