/*
Write a program that removes all occurrences of the element elem_to_remove from a vec.

The output of the program should be the new vec with the element removed.

If the element is not found in the vec, print the message "Not Found".

If the vec is empty, print "Empty List".

    {vec}              {element to remove}      {output}
    [1,2,3,4]           2                       [1,3,4]
    [3,3,2,1]           3                       [2,1]
    ["a","b","c","b"]   "b"                     ["a","c"]
    [3,4,5,6]           7                       "Not Found"
    []                  0                       "Empty Vec"

*/

use core::fmt;
use std::{any::Any, fmt::Debug};

fn remove_thing_from_vec<T, U>(data: Vec<T>, element: U)
where
    T : Any + PartialEq<U> + std::fmt::Display + Debug + std::clone::Clone,
    U : Any + PartialEq<T> + std::fmt::Display + Debug,
{
    
    // case empty
    if data.is_empty() {
        println!("\"Empty Vec\"");
        return;
    }
    // case not found
    let check_is_contain_element = data.iter().position(|x| *x == element);
    match check_is_contain_element {
        Some(_) => (),
        None => {
            println!("{:?}","Not Found".to_owned());
            return;},
    }

    // filter those match
    let filter_data: Vec<T> = data.iter()
                                .filter(|&x| *x != element)
                                .cloned()
                                .collect();
    
    println!("{:?}",filter_data);
  

}

#[cfg(test)]
mod test{
    use super::remove_thing_from_vec;

    #[test]
    fn test_1() {
        //[1,2,3,4]           2                       [1,3,4]
        let data = vec![1,2,3,4];
        remove_thing_from_vec(data, 2)
    }

    #[test]
    fn test_2() {
        //[3,3,2,1]           3                       [2,1]
        let data = vec![3,3,2,1];
        remove_thing_from_vec(data, 3);
    }

    #[test]
    fn test_3() {
        // ["a","b","c","b"]   "b"                     ["a","c"]
        let data = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), "b".to_owned()];
        remove_thing_from_vec(data, "b");
    }

    #[test]
    fn test_4() {
        //   [3,4,5,6]           7                       "Not Found"
        let data = vec![3, 4, 5, 6];
        remove_thing_from_vec(data, 7);
    }

    #[test]
    fn test_5() {
        // []                  0                       "Empty Vec"
        let data: Vec<i32> = vec![];
        remove_thing_from_vec(data, 0);
    }

}