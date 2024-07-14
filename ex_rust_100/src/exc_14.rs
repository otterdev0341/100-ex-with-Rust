/*
    Write a program that reverses the individual words in the string s and changes their capitalization. Uppercase letters should be printed in lowercase and vice versa.

    Assume that the string only contains letters and spaces are used to separate words.

    ** exc_3 : reverse_string

    {String}            {Output}
    "Hello World"       "OLLEh DLROw"
    "Python is Awesome" "NOHTYp SI EMOSEWa"
*/

use crate::exc_3::reverse_string;

fn reverse_then_switch_capialized(context: String) -> String {
    let worlds: Vec<_> = context.split(' ').collect();
    let mut switch_vec: Vec<String> = vec![];
    let mut result_vec: Vec<_> = vec![];
    
    for world in worlds {
        let temp = reverse_string(world.to_owned());
        let temp = switch_captial(temp);
        switch_vec.push(temp);
    }

    for data in switch_vec {
        result_vec.push(data);
    }

    return result_vec.join(" ");
}

fn switch_captial(context: String) -> String {
   
   let mut result_vec: Vec<char> = vec![];
   for ch in context.chars() {
    if ch.is_lowercase() {
        result_vec.push(ch.to_uppercase().next().unwrap())
    } else {
        result_vec.push(ch.to_lowercase().next().unwrap())
    }
   }
    return result_vec.iter().collect();
}

#[cfg(test)]
mod test {
    use std::result;

    use super::reverse_then_switch_capialized;

    #[test]
    fn test_1() {
        // "Hello World"       "OLLEh DLROw"
        let context = "Hello World".to_owned();
        let expect = "OLLEh DLROw".to_owned();
        let result = reverse_then_switch_capialized(context);
        assert_eq!(expect,result);
    }
    #[test]
    fn test_2() {
        // "Python is Awesome" "NOHTYp SI EMOSEWa"
        let context = "Python is Awesome".to_owned();
        let expect = "NOHTYp SI EMOSEWa".to_owned();
        let result = reverse_then_switch_capialized(context);
        assert_eq!(expect, result);
    }
}

