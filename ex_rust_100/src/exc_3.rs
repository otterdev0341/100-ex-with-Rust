/*
    Write program to reversed version of a string
    The program must preserve uppercase and lowercase letters
    if string is empty, print it intact

    {input}     {output}
    "Hello"      "olleH"
    "Wo"         "oW"
      ""          ""
*/

fn reverse_string(context : String) -> String {
    let mut length_of_string = context.len();
    if length_of_string == 0 {
        return String::from("");
    } else {
        let vec_char: Vec<char> = context.chars().collect();
        let mut result_char = vec![];
        while length_of_string != 0 {
            result_char.push(vec_char[length_of_string - 1]);
            length_of_string = length_of_string - 1;
        }
        return result_char.iter().collect();
    }
}

#[cfg(test)]
mod test_reverse_string{
    use super::reverse_string;


    #[test]
    fn test_1() {
        //"Hello"      "olleH"
        let context = "Hello".to_owned();
        let expect = "olleH".to_owned();
        let result_from_function = reverse_string(context);

        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_2() {
        //  "Wo"         "oW"
        let context = "Wo".to_owned();
        let expcct = "oW".to_owned();
        let result_from_function = reverse_string(context);
        assert_eq!(expcct,result_from_function);
    }

    #[test]
    fn test_3() {
        // ""          ""
        let context = "".to_owned();
        let expect = "".to_owned();
        let result_from_function = reverse_string(context);
        assert_eq!(expect, result_from_function);
    }

}