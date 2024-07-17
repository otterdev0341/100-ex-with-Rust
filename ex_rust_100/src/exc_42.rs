/*
Write a  program that creates and displays a HashMap that maps each letter in a string to how many times the character occurs in the string (its frequency).

The HashMap should only include the characters in the string. 

The test should be case-insensitive ("A" should be counted as "a").

The keys in the HashMap should be lowercase letters.

Only include letters in the HashMap.

🔹 Expected Output:
Example 1:

For the string:

"Hello, World"

The output should be this HashMap:

{"h": 1, "e": 1, "l": 3, "o": 2, "w": 1, "r": 1, "d": 1}
Each letter is mapped to its corresponding frequency.

Example 2:

"Excellent"

The output should be this HashMap:

{"e": 3, "x": 1, "c": 1, "l": 2, "n": 1, "t": 1}

*/

use std::collections::HashMap;

fn count_char_in_string(context: String) {
    
    let lower_context = context.to_lowercase();
    let vec_char: Vec<_> = lower_context.chars()
                                        .filter(|x|x.is_alphabetic())
                                        .collect();

    let mut vec_tuple: Vec<(String,u32)> = vec![];

    
    for ch in vec_char  {
        vec_tuple.push((ch.to_string(),1));
    }

    let mut map_result: HashMap<String, i32> = HashMap::new();
    for (ch, val) in vec_tuple.into_iter() {
        
        if let Some(to_update) = map_result.get_mut(&ch) {
            *to_update = *to_update + 1;
        }
        if !map_result.contains_key(&ch) {
            map_result.insert(ch, val as i32);
        }
    }
    println!("{:?}",map_result);
}

#[cfg(test)]
mod test {
    use super::count_char_in_string;


    #[test]
    fn test_1() {
        let context = "Hello, World".to_owned();
        count_char_in_string(context);
    }

    #[test]
    fn test_2() {
        let context = "Excellent".to_owned();
        count_char_in_string(context);
    }
}