/*
Write a  program that prints the digits of a number in reverse order on the same line.

If the number only has one digit, print this digit.

        {number}            {output}
        1                   1
        123                 321
        3456                6543
        1111                1111
        2662                2662
*/

fn reverse_number(number: i32) -> i32 {

    let is_negative = number< 0;
    let mut number_str = number.abs().to_string();
    let reverse_str: String = number_str.chars().rev().collect();
    let reverse_number: i32 = reverse_str.parse().unwrap();

    if is_negative {
        -reverse_number
    } else {
        reverse_number
    }

}

#[cfg(test)]
mod test {
    use crate::exc_66::reverse_number;


    #[test]
    fn test_1() {
        // 1        1
        assert_eq!(1,reverse_number(1));
    }


    #[test]
    fn test_2() {
        //   123                 321
        assert_eq!(321,reverse_number(123));
    }


    #[test]
    fn test_3() {
        //  3456                6543
        assert_eq!(6543,reverse_number(3456));
    }


    #[test]
    fn test_4() {
        //  1111                1111
        assert_eq!(1111,reverse_number(1111));
    }


    #[test]
    fn test_5() {
        // 2662                2662
        assert_eq!(2662,reverse_number(2662));
    }
}