/*
    Write a program that checks if a list is empty or not.

    If the list is empty, print "Empty". Else, print "Not Empty".

    {Vec}                  {Output}
    []                      "Empty"
    [4]                     "Not Empty"
    [4, 5, 6, 7]            "Not Empty"
*/

use std::any::Any;

fn is_our_vec_empty<T: Any>(data: Vec<T>) {
    if data.is_empty() {
        println!("Empty");
    } else {
        println!("Not Empty");
    }
}

#[cfg(test)]
mod test {
  

    use super::is_our_vec_empty;


    #[test]
    fn test_1() {
        let data: Vec<i32> = vec![];
        is_our_vec_empty(data);
    }

    #[test]
    fn test_2() {
        // [4]                     "Not Empty"
        let data: Vec<i32> = vec![4];
        is_our_vec_empty(data);
    }

    #[test]
    fn test_3() {
        //[4, 5, 6, 7]            "Not Empty"
        let data = vec![4, 5, 6, 7];
        is_our_vec_empty(data);
    }
}