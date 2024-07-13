/*
    Write a program that prints the string s without the characters located at even indices.
    If the string is empty or only has one character, print it intact.

    {input}     {output}
    "Coding"     "oig"
    "Pizza"      "iz"
    "Python"     "yhn"
    "A"          "A"
    ""           ""
*/

fn no_more_even(context: String) -> String {

    let mut result_vec: Vec<char> = vec![];
    let array_chars: Vec<char> = context.chars().collect();
    let len = context.len();
    let mut index = 0;
    if len <= 1 {
        if len == 1 {
            result_vec.push(array_chars[0]);
            return  result_vec.iter().collect();
        } else {
            return "".to_owned();
        }
    }
    while let Some(_) = array_chars.iter().next() {
        if index == 0 || index / 2 == 0 {
            result_vec.push(array_chars[index]);
        }
        index = index + 1
    }
    
    return result_vec.iter().collect();
}

#[cfg(test)]
mod test_ex_5 {
    use super::no_more_even;


    #[test]
    fn test_1() {
        //"Coding"     "oig"
        let context = "Coding".to_owned();
        let expect = "oig".to_owned();
        let result_from_function = no_more_even(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_2() {
        //  "Pizza"      "iz"
        let context = "Pizza".to_owned();
        let expect = "oig".to_owned();
        let result_from_function = no_more_even(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_3() {
        //"Python"     "yhn"
        let context = "Python".to_owned();
        let expect = "yhn".to_owned();
        let result_from_function = no_more_even(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_4() {
        // "A"          "A"
        let context = "A".to_owned();
        let expect = "A".to_owned();
        let result_from_function = no_more_even(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_5() {
        // "A"          "A"
        let context = "".to_owned();
        let expect = "".to_owned();
        let result_from_function = no_more_even(context);
        assert_eq!(expect,result_from_function);
    }

}