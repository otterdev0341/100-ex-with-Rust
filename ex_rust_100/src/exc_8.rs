/*
    write program that print string s with char curr_char replace by new_char
    curr_char and new_char are variables that contain strings with a single character.
    You may assume that new_char will not be an empty string.
    The match must be case-sensitive (do not replace lowercase letters if curr_char is uppercase).
    If no match is found, print the initial string.

    {String}       {curr_char}      {new_char}      {output}
    "Hello"         "l"                 "s"         "Hesso"
    "World"         "W"                 "A"         "Aorld"
    "Python"        "P"                 "x"         "xython"
    "Python"        "p"                 "a"         "Python"
*/

fn replace_string_with_c(context: String, c: char, p: char) -> String {
    let mut result: Vec<char> = vec![];
    for ch in context.chars() {
        if ch == c {
            result.push(p);
            continue;
        }
        result.push(ch);
    }
    return  result.iter().collect();
}

#[cfg(test)]
mod test_exc_8 {
    use super::replace_string_with_c;

    #[test]
    fn test_1() {
        // "Hello"         "l"                 "s"         "Hesso"
        let context = "Hello".to_owned();
        let curr_char = 'l';
        let new_char = 's';
        let expect = "Hesso".to_owned();
        let result = replace_string_with_c(context, curr_char, new_char);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        //  "World"         "W"                 "A"         "Aorld"
        let context = "World".to_owned();
        let curr_char = 'W';
        let new_char = 'A';
        let expect = "Aorld".to_owned();
        let result = replace_string_with_c(context, curr_char, new_char);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "Python"        "P"                 "x"         "xython"
        let context = "Python".to_owned();
        let curr_char = 'P';
        let new_char = 'x';
        let expect = "xython".to_owned();
        let result = replace_string_with_c(context, curr_char, new_char);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_4() {
        // "Python"        "p"                 "a"         "Python"
        let context = "Python".to_owned();
        let curr_char = 'p';
        let new_char = 'a';
        let expect = "Python".to_owned();
        let result = replace_string_with_c(context, curr_char, new_char);
        assert_eq!(expect,result);
    }


}