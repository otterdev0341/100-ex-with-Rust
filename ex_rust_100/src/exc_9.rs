/*
    write program that prints a version of string s with all commas replaced by dots.
    {String}            {Output}
    "Hello, World!"     "Hello. World!"
    "3,456,344"         "3.456.344"
*/

fn replace_comma_by_dot(context: String) -> String {
    
    let mut result: Vec<char> = vec![];
    for ch in context.chars() {
        if ch == ',' {
            result.push('.');
            continue;
        }
        result.push(ch);
    }
    return result.iter().collect();
}

#[cfg(test)]
mod test{
    use super::replace_comma_by_dot;

    #[test]
    fn test_1() {
        //"Hello, World!"     "Hello. World!"
        let context = "Hello, World!".to_owned();
        let expect = "Hello. World!".to_owned();
        let result = replace_comma_by_dot(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // "3,456,344"         "3.456.344"
        let context = "3,456,344"  .to_owned();
        let expect = "3.456.344".to_owned();
        let result = replace_comma_by_dot(context);
        assert_eq!(expect,result);
    }
}