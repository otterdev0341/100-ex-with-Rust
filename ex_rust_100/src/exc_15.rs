/*
    Write a  program to count the number of repeated characters in the string s.

    The program must print the total number of repeated characters and a message on the next line displaying the repeated characters separated by a space and sorted alphabetically.

    If there are no repeated characters in the string, print 0 as the total count and "" on the next line.


    {String}        {output}
    "Hello"         (1,"l")
    "Corporation"   (2,"o r")
    "Python"        (0, "")
    */

use std::{collections::HashMap, iter};





pub fn repeated_char(context : String) -> (i32, String) {
    
    let mut vec: Vec<(String, i32)> = Vec::new();
    let mut result: HashMap<String, i32> = HashMap::new();
    let mut final_result: (i32, String) = (0, "".to_owned());
    let mut dupicate_char: Vec<String> = vec![];
    for x in context.chars() {
        vec.push((x.to_string(),1));
    }
 
    for (x, y) in vec.iter_mut() {
        if let Some(count) = result.get_mut(x) {
            *count += 1;
        } else {
            result.insert(x.clone(), *y);
        }
    }

    let result: HashMap<String, i32> = result.into_iter()
                                             .filter(|&(_, v)| v > 1).collect();
           
    let dupicate_count = result.len();
    for (x, _y) in result.into_iter() {
        dupicate_char.push(x.clone());
    }
    dupicate_char.sort_by(|a, b| a.cmp(b));
    final_result.0 = dupicate_count as i32;
    final_result.1 = dupicate_char.join(" ");
    return final_result;

}

#[cfg(test)]
mod test {
    

    use super::repeated_char;

    #[test]
    fn test_1() {
        // Hello (1, "l")
        let context = "Hello".to_owned();
        let expect = (1,"l".to_owned());
        let result = repeated_char(context);
        assert_eq!(expect.0, result.0);
        assert_eq!(expect.1, result.1);
    }

    #[test]
    fn test_2() {
         // Corporation (2, "o r")
         let context = "Corporation".to_owned();
         let expect = (2,"o r".to_owned());
         let result = repeated_char(context);
         assert_eq!(expect.0, result.0);
         assert_eq!(expect.1, result.1);
    }

    #[test]
    fn test_3() {
        // Python (0, "")
        let context = "Python".to_owned();
         let expect = (0,"".to_owned());
         let result = repeated_char(context);
         assert_eq!(expect.0, result.0);
         assert_eq!(expect.1, result.1);
    }
}