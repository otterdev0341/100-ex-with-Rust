/*
    write program copy string without space
    world should connect with final string
    if the string not contain space, print it intact

    {String}            {Output}
    "Hello, World!"     "Hello,World!"
    "Have a great day"  "Haveagreatday"
    "Python"            "Python"

*/
fn remove_space_from_string(context: String) -> String {
    let space = ' ';
    let mut result: Vec<char> = vec![];
    
    for ch in context.chars() {
        if ch == space {
            continue;
        }
        result.push(ch);
    }
    
    return result.iter().collect();
}

#[cfg(test)]
mod test {
    use super::remove_space_from_string;


    #[test]
    fn test_1() {
        // "Hello, World!"     "Hello,World!"
        let context = "Hello, World!".to_owned();
        let expect = "Hello,World!".to_owned();
        let result = remove_space_from_string(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // "Have a great day"  "Haveagreatday"
        let context = "Have a great day".to_owned();
        let expect = "Haveagreatday".to_owned();
        let result = remove_space_from_string(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "Python"            "Python"
        let context = "Python".to_owned();
        let expect = "Python".to_owned();
        let result = remove_space_from_string(context);
        assert_eq!(expect,result);
    }
}