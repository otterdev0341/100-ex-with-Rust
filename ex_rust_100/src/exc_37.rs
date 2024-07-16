/*
Write a  program that prints the largest value in a HashMap.

If the HashMap is empty, print None.

You may assume that the values are numeric.

    {input}                     {Output}
    {"a":4, "b":3, "c":7}       7
    {"a":4, "b":6}              6
    {"a":4, "b":4}              4
    {}                          0
*/

use std::{any::Any, collections::HashMap, fmt::{Debug, Display}, hash::Hash};

fn the_largest_val_in_hash<T>(data: HashMap<T,usize>) -> usize
where 
    T : Display + Debug + Hash + Eq + Clone + Any,
{
    if data.is_empty() {
        return 0;
    }
    let val_vec: Vec<_> = data.into_values().collect();
    let result = val_vec.iter().max();
    match result {
        Some(x) => return *x,
        None => {return 0},
    }

}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::the_largest_val_in_hash;

    #[test]
    fn test_1() {
        // {"a":4, "b":3, "c":7}       7
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 3);
        data.insert("c".to_owned(), 7);

        let expect: usize = 7;
        let result = the_largest_val_in_hash(data);
        assert_eq!(expect, result);
        
    }

    #[test]
    fn test_2() {
        // {"a":4, "b":6}              6
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 6);

        let expect: usize = 6;
        let result = the_largest_val_in_hash(data);
        assert_eq!(expect, result);
    }
    #[test]
    fn test_3() {
        // {"a":4, "b":4}              4
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 4);

        let expect: usize = 4;
        let result = the_largest_val_in_hash(data);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_4() {
        //{}                          0
        let data: HashMap<String, usize> = HashMap::new();
        let expect: usize = 0;
        let result = the_largest_val_in_hash(data);
        assert_eq!(expect, result);
    }
}