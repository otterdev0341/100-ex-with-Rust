/*
Write a  program that prints if a number num is either "Positive", "Negative", or "Zero".

    {number} {output}
    3           "Positive"
    -1          "Negative"
    0           "Zero"
*/

fn pzn_num(number: i32) {

    if number == 0 {
        println!("Zero");
    } else {
        if number > 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    }
}

#[cfg(test)]
mod test {
    use super::pzn_num;


    #[test]
    fn test_1() {
        let number = 3;
        pzn_num(number);
    }

    #[test]
    fn test_2() {
        let number = -1;
        pzn_num(number);
    }

    #[test]
    fn test_3() {
        let number = 0;
        pzn_num(number);
    }
}