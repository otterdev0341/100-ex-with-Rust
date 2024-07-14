/*
    Write a program that checks if the string s starts with the sequence of characters denoted by the variable prefix.

    If it does, print True. Else, print False.

    This test should be case sensitive. For example, "A" should not be equivalent to "a".

    If the length of the prefix is greater than the length of the string, print False.

    {String}            {Prefix}        {Output}
    "Hello"             "He"            true
    "Coding"            "Con"           false
    "Nora"              "Circum"        false
    
*/

fn sequence_of_string(context: String, prefix: String) -> bool {
    let mut result = false;
    let mut couter = 1;
    let prefix_len = prefix.len();
    if context.len() < prefix_len {
        return result;
    }
    for (a, b) in context.chars().zip(prefix.chars()) {
        if a == b {
            couter = couter + 1;
            if couter == prefix_len {
                result = true;
                return result;
            }
        }
        couter = 0;
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::sequence_of_string;

    #[test]
    fn test_1() {
        //  "Hello"             "He"            true
        let context = "Hello".to_owned();
        let prefix = "He".to_owned();
        let expect = true;
        let result = sequence_of_string(context, prefix);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // "Coding"            "Con"           false
        let context = "Coding".to_owned();
        let prefix = "Con".to_owned();
        let expect = false;
        let result = sequence_of_string(context, prefix);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "Nora"              "Circum"        false
        let context = "Nora".to_owned();
        let prefix = "Circum".to_owned();
        let expect = false;
        let result = sequence_of_string(context, prefix);
        assert_eq!(expect,result);
    }
}