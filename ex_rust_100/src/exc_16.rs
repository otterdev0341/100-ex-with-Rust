/*
    Write a program to convert a string s to lowercase, sort the characters of each word in alphabetical order, and print the resulting string.

    You may assume that the string only contains letters and spaces to separate the words.

    Spaces should be preserved in the final string.

    {String}            {Output}
    "Hello World"       "ehllo dlorw"
    "Wonderful World"   "deflnoruw dlorw"
*/

fn reverse_sort(context: String) -> String {
    let data = context.to_lowercase();
    let vec_data: Vec<_> = data.split(" ").collect();
    let mut result_vec: Vec<String> = vec![];

    for element in vec_data.iter() {
        let mut temp: Vec<_> = element.chars().collect();
        temp.sort_by(| a, b | a.cmp(b));
        // println!("temp : {:?}",temp);
        result_vec.push(temp.iter().collect());
    }

    return result_vec.join(" ");
}

#[cfg(test)]
mod test_exc_16 {
    use super::reverse_sort;

    #[test]
    fn test_1() {
        // "Hello World"       "ehllo dlorw"
        let context = "Hello World".to_owned();
        let expect = "ehllo dlorw".to_owned();
        let result = reverse_sort(context);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_2() {
        // "Wonderful World"   "deflnoruw dlorw"
        let context = "Wonderful World".to_owned();
        let expect = "deflnoruw dlorw".to_owned();
        let result = reverse_sort(context);
        assert_eq!(expect, result);
    }
}

