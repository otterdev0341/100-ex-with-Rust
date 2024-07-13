/*
    - write program that prints the character at index i in the string s
    - if index is out of range, should print "i is out of range"
    - if string is empty, should print "Empty String"

    {String} {input} {Output}
    "Hello"     2       "l"
    "Pizza"     4       "a"
       ""       3       "Empty String"
    "World"     15      "i is out of range"
*/
use crate::exc_1::length_of_string;

fn get_char_from_specific_index(context: String, i: u32) -> String {
    let length_of_context = length_of_string(&context);
    if i >= length_of_context && context != "" {
        return String::from("i is out of range");
    } else if context == ""{
        return String::from("Empty String");
    } else {
        
        let char = &context.chars().nth(i as usize);
        match char {
            Some(c) => return c.to_string(),
            None => return "Error ocur function".to_owned(),
        }
      
    }
}

#[cfg(test)]
mod test_exc_2 {
    use crate::exc_2::get_char_from_specific_index;

    #[test]
    fn test_1() {
        //"Hello"     2       "l"
        let context = "Hello".to_owned();
        let index = 2;
        let expect_result = "l".to_owned();
        assert_eq!(expect_result,get_char_from_specific_index(context, index))
    }

    #[test]
    fn test_2() {
        //"Pizza"     4       "a"
        let context = "Pizza".to_owned();
        let index = 4;
        let expect_result = "a".to_owned();
        assert_eq!(expect_result, get_char_from_specific_index(context, index));
    }

    #[test]
    fn test_3() {
        // ""       3       "Empty String"
        let context = "".to_owned();
        let index = 3;
        let expect_result = "Empty String".to_owned();
        assert_eq!(expect_result,get_char_from_specific_index(context, index));
    }

    #[test]
    fn test_4() {
        //  "World"     15      "i is out of range"
        let context = "World".to_owned();
        let index = 15;
        let expect_result = "i is out of range".to_owned();
        assert_eq!(expect_result, get_char_from_specific_index(context, index));
    }
}

