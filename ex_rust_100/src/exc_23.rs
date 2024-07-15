/*
Write a  program that removes duplicate elements from a vec, only keeping one occurrence of each element in the list.

The original list should be mutated (modified).

The program must print the final version of the list.

    {input}                 {output}
    [1,1,2,3,4,4]           [1,2,3,4]
    ["a","a","b","a"]       ["a","b"]
    [1,2,3]                 [1,2,3]
    []                      []
*/

use core::fmt;
use std::{any::Any, collections::HashSet};


fn remove_dup_in_vec<T>(data: &mut Vec<T>)
where
    T : Any + std::cmp::Eq + std::hash::Hash + fmt::Debug + Clone,
{
    // empty case
    if data.is_empty() {
        println!("[]");
        return;
    }

    let mut uniques = HashSet::new();
    data.retain(|e| uniques.insert(e.clone()));
    
    println!("{:?}",data);

    
}

#[cfg(test)]
mod test {
    use super::remove_dup_in_vec;


    #[test]
    fn test_1() {
        // [1,1,2,3,4,4]           [1,2,3,4]
        let mut data = vec![1,1,2,3,4,4];
        remove_dup_in_vec(&mut data);
    }

    #[test]
    fn test_2() {
        //["a","a","b","a"]       ["a","b"]
        let mut data: Vec<String> = vec!["a".to_owned(), "a".to_owned(), "b".to_owned(), "a".to_owned()];
        remove_dup_in_vec(& mut data);
    }

    #[test]
    fn test_3() {
        // [1,2,3]                 [1,2,3]
        let mut data : Vec<_> = vec![1,2,3];
        remove_dup_in_vec(&mut data);
    }

    #[test]
    fn test_4() {
        // []                      []
        let mut data : Vec<i32> = vec![];
        remove_dup_in_vec(&mut data);
    }

}
