/*
    write a program that check if a string only contains numbers
    if it does print True, Else print False

    {input}     {output}
    "Hello"     False
    "4567"      True
    "Hello59"   False
    ""          False
*/

fn is_it_only_digits(context: String) -> bool {

    let mut result = true;
    if context.is_empty() {
        result = false;
        return result;
    }
    for c in context.chars() {
        let temp = c as u8;
        if temp >= 58 || temp <= 47 {
            result = false;
            return result;
        }
    }
    
    return result;
}

#[cfg(test)]
mod test_exc_6 {
    use super::is_it_only_digits;


    #[test]
    fn test_1() {
        //  "Hello"     False
        let context = "Hello".to_owned();
        let expect = false;
        let result = is_it_only_digits(context);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_2() {
        // "4567"      True
        let context = "4567".to_owned();
        let expect = true;
        let result = is_it_only_digits(context);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_3() {
        // "Hello59"   False
        let context = "Hello59".to_owned();
        let expect = false;
        let result = is_it_only_digits(context);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_4() {
        //""          False
        let context = "".to_owned();
        let expect = false;
        let result = is_it_only_digits(context);
        assert_eq!(expect, result);
    }


}