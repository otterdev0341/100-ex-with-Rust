/*

Write a program that checks if a key exists in a hashmap or not.

If the key exists in the hashmap, print True. Else, print False.

The key should be stored in the variable key.
    {Hashmap}               {Key}           {Output}
    {"a":4, "b":6}           "a"            true
    {"a:4","b":6}            "c"            false
    {}                       "d"            false
*/

use std::{any::Any, collections::HashMap, hash::Hash};

fn is_key_exist<T>(data: HashMap<T,usize>,key: T) -> bool
where
    T : Any + Clone + Eq + Hash + Sized,
{
    let mut result = false;
    if data.is_empty() {
        
        return result;
    }
    if data.contains_key(&key) {
        result = true;
    }
    return  result;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::is_key_exist;


    #[test]
    fn test_1() {
        // {"a":4, "b":6}           "a"            true
        let mut data: HashMap<String,usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 6);
        let key = "a".to_owned();
        let expect = true;
        let result = is_key_exist(data, key);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        //{"a:4","b":6}            "c"            false
        let mut data: HashMap<String,usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 6);
        let key = "c".to_owned();
        let expect = false;
        let result = is_key_exist(data, key);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        //{}                       "d"            false
        let data: HashMap<String,usize> = HashMap::new();
        
        let key = "d".to_owned();
        let expect = false;
        let result = is_key_exist(data, key);
        assert_eq!(expect,result);
    }
}