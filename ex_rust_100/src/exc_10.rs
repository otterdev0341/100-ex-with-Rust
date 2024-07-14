/*
    write program check if all string is alphabet (cas-sensitive a == A)
    if it does, print true else print false
    before compare, should convert to lowercase
    may assum the string doesn't contain any other symbol, only space
    
    {string}        {result}
    "abc def"            true
    "abc88"             false
    "HeAo"              true
    "1234"              false
*/


fn is_alpha_ignore_space(context: String) -> bool {
    
    let mut result = true;
    let lower_context = context.to_lowercase();

    for ch in lower_context.chars() {
        let temp = ch as usize;
        if temp == 32 {
            continue;
        }
        if temp < 97 || temp > 122 {
            result = false;
            return result;
        }
    }
    return result;
}

#[cfg(test)]
mod test{
    use super::is_alpha_ignore_space;


    #[test]
    fn test_1() {
        // "abcdef"            true
        let context = "abc def".to_owned();
        let expect = true;
        let result = is_alpha_ignore_space(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // "abc88"             false
        let context = "abc88".to_owned();
        let expect = false;
        let result = is_alpha_ignore_space(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // "HeAo"              true
        let context = "HeAo".to_owned();
        let expect = true;
        let result = is_alpha_ignore_space(context);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_4() {
        // "1234"              false
        let context = "1234".to_owned();
        let expect = false;
        let result = is_alpha_ignore_space(context);
        assert_eq!(expect, result);
    }
}