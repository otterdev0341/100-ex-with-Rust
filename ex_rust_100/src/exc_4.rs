/*
    first and last 3
    wirte program to print first and last three char from string s 
    if string has less thatn six chars, print empty string

    {input}     {output}
    "Blue"          ""
    "Wonderful"     "Wonful"
    "Amazing"       "Amaing"
*/

fn first_last_three(context: String) -> String {

    let length_of_context = context.len() - 1;
    let mut start = 0;
    let mut cursor = length_of_context - 2;
    let mut result_vec: Vec<char> = vec![];
    if length_of_context <= 5 {
        return "".to_owned();
    } else {
        let chars_vec: Vec<char> = context.chars().collect();
        while start <= 2 {
            result_vec.push(chars_vec[start]);
            start = start + 1;
        }
        while cursor <= length_of_context {
            result_vec.push(chars_vec[cursor]);
            cursor = cursor + 1;
        }
        return result_vec.iter().collect();
    }

}

#[cfg(test)]
mod test_exc_4 {
    use super::first_last_three;

    #[test]
    fn test_1() {
        //"Blue"          ""
        let context = "Blue".to_owned();
        let expect = "".to_owned();
        let result_from_function = first_last_three(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_2() {
        //"Wonderful"     "Wonful"
        let context = "Wonderful".to_owned();
        let expect = "Wonful".to_owned();
        let result_from_function = first_last_three(context);
        assert_eq!(expect,result_from_function);
    }

    #[test]
    fn test_3() {
        // "Amazing"       "Amaing"
        let context = "Amazing".to_owned();
        let expect = "Amaing".to_owned();
        let result_from_function = first_last_three(context);
        assert_eq!(expect,result_from_function);
    }
}