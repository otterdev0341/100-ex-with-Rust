/*
    create programe that print string s without chartecter ant index n
    if index n out of range, print string s
    if string s is empty print the string s intact
    0 1 2 3 4
    {String}        {n}      {output}
    "Hello"          1        "Hllo"
    "World"          3        "Word"
    "Dog"            15       "Dog"
    ""               2        ""
 */

 fn no_n_in_string(context: String, n: u16) -> String {

    let mut result: Vec<char> = vec![];
    let mut cursor = 0;
    if context.is_empty() {
        return "".to_owned();
    }
    if context.len() - 1 < n as usize {
        return context;
    }
    for c in context.chars() {
        if cursor == n {
            cursor = cursor + 1;
            continue;
        }
        result.push(c);
        cursor = cursor + 1;
    }


    return result.iter().collect();
 }

 #[cfg(test)]
 mod test_exc_7 {
    use super::no_n_in_string;

    #[test]
    fn test_1() {
        //"Hello"          1        "Hllo"
        let context = "Hello".to_owned();
        let n = 1;
        let expect = "Hllo".to_owned();
        let result = no_n_in_string(context, n);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // "World"          3        "Word"
        let context = "World".to_owned();
        let n = 3;
        let expect = "Word".to_owned();
        let result = no_n_in_string(context, n);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "Dog"            15       "Dog"
        let context = "Dog".to_owned();
        let n = 15;
        let expect = "Dog".to_owned();
        let result = no_n_in_string(context, n);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_4() {
        // ""               2        ""
        let context = "".to_owned();
        let n = 2;
        let expect = "".to_owned();
        let result = no_n_in_string(context, n);
        assert_eq!(expect,result);
    }
 }