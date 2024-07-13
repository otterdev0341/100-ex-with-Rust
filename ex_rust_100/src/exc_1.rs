// -> Write program that prints the length of string s
// Expect :
/*
    "" => 0
    "H" => 1
    "Hello" => 5
    "Amazing" => 7
*/

pub fn length_of_string(context: &String) -> u32{
    
    let mut array_char = context.chars();
    let mut length = 0;
    while let Some(_) = array_char.next(){
        length = length + 1;
    }
    return length;
}

#[cfg(test)]
mod test_one {
    use crate::exc_1::length_of_string;


    #[test]
    fn test_1() {
        let test_1 = "".to_owned();
        let expect_result = 0;

        assert_eq!(expect_result,length_of_string(&test_1));
    }
    
    #[test]
    fn test_2() {
        let test_2 = "H".to_owned();
        let expect_result = 1;
        assert_eq!(expect_result,length_of_string(&test_2));
    }

    #[test]
    fn test_3() {
        let test_3 = "Hello".to_owned();
        let expect_result = 5;
        assert_eq!(expect_result,length_of_string(&test_3));
    }

    #[test]
    fn test_4(){
        let test_4 = "Amazing".to_owned();
        let expect_result = 7;
        assert_eq!(expect_result, length_of_string(&test_4));
    }
}