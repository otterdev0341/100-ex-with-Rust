/*
Write a  program that checks if all values in a HashMap are equal.

If they are, print True. Else, print False.

If the HashMap is empty, print "Empty".

    {Data}                          {output}
    {"a": 4, "b":4, "c":4}          true
    {"a":4, "b":6, "c":4}           false
    {"a":4, "b":6, "c": 10}         false
    {}                              "Empty"

*/

use std::{collections::HashMap, fmt::{Debug, Display}, hash::Hash};
use std::any::Any;
fn is_all_value_are_equal<T>(data: HashMap<T,usize>) -> String
where
    T : Any + Eq + Hash + Display + Debug + Clone,
{
    if data.is_empty() {
        return "Empty".to_owned();
    }

    let mut result = "True".to_owned();
    let mut temp_vec: Vec<(T,usize)> = vec![];
    // tranform hashmap into vec[(key,val)]
    for (key, val) in data.iter() {
        temp_vec.push((key.clone(), *val));
    }
    // take val of 1st element to use as base case
    let base_value = temp_vec[0].1;
    // compare to other
    for (_key, val) in temp_vec.iter() {
        if *val != base_value {
            return "False".to_owned();
        }
    }
   
    return result;
    
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::is_all_value_are_equal;



    #[test]
    fn test_1() {
        // {"a": 4, "b":4, "c":4}          True
        let mut data:HashMap<String,usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 4);
        data.insert("c".to_owned(), 4);
        
        let expect = "True".to_owned();
        let result = is_all_value_are_equal(data);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_2() {
        // {"a":4, "b":6, "c":4}           False
        let mut data:HashMap<String,usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 6);
        data.insert("c".to_owned(), 4);

        let expect = "False".to_owned();
        let result = is_all_value_are_equal(data);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_3() {
        // {"a":4, "b":6, "c": 10}         false
        let mut data:HashMap<String,usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 6);
        data.insert("c".to_owned(), 10);

        let expect = "False".to_owned();
        let result = is_all_value_are_equal(data);
        assert_eq!(expect,result);
    }

    #[test]
    fn test_4() {
        //{}                              "Empty"
        let data:HashMap<String,usize> = HashMap::new();
        let expect = "Empty".to_owned();
        let result = is_all_value_are_equal(data);
        assert_eq!(expect,result);
    }
}