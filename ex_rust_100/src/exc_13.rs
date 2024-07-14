/*
    Write a Python program that checks if the string s ends with a specific sequence of characters denoted by the variable suffix.

    If it does, print True. Else, print False.

    This test should be case sensitive. Therefore, "A" should not be equivalent to "a".

    If the length of the suffix is greater than the length of the string, print False.

    {String}            {Suffix}            {Output}
    "Hello"             "ello"              true
    "Coding"            "eng"               false
    "Nora"              "rowing"            "false"

*/

fn is_end_with_suffix(context: String, suffix: String) -> bool {
    let mut result = false;
    let len_of_suffix = suffix.len();
    let len_of_context = context.len();
    let mut cursor = 0;

    if len_of_context <= len_of_suffix {
        return result;
    } else {
        cursor = len_of_context - len_of_suffix;
    }
    

    let sub_context = &context[cursor..len_of_context];
    if sub_context == suffix {
        result = true;
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::is_end_with_suffix;


    #[test]
    fn test_1() {
        //"Hello"             "ello"              true
        let context = "Hello".to_owned();
        let suffix = "ello".to_owned();
        let expect = true;
        let result = is_end_with_suffix(context, suffix);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        //   "Coding"            "eng"               false
        let context = "Coding".to_owned();
        let suffix = "eng".to_owned();
        let expect = false;
        let result = is_end_with_suffix(context, suffix);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "Nora"              "rowing"            "false"
        let context = "Nora".to_owned();
        let suffix = "rowing".to_owned();
        let expect = false;
        let result = is_end_with_suffix(context, suffix);
        assert_eq!(expect,result);
    }
}

