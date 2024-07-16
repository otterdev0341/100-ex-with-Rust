/*
Write a  program that prints a vec with the elements that vecA and vecB have in common.

If they don't have any elements in common, print an empty list.

The program should not assume that the lists have the same length.

You may assume that each element will only appear once in each list.

    {pointA}                {pointB}            {Output}
    [1,2,3]                 [1,2,3]             [1,2,3]
    [4,5,6]                 [1,4,5]             [4,5]
    [3,4,5]                 [1,2,3]             [3]
    [4,5,6]                 [1,2,3]             []

*/

use std::{any::Any, fmt::Debug};

fn find_common_in_vec<T>(vec_a: Vec<T>, vec_b: Vec<T>)
where 
    T : Any + PartialEq + std::fmt::Display + Clone + Debug,
{
    let common_vec: Vec<_> = vec_a.iter()
                                  .filter(| x| vec_b.contains(x))
                                  .cloned()
                                  .collect();
    if common_vec.is_empty() {
        println!("[]");
    } else {
        println!("{:?}",common_vec);
    }
    
}

#[cfg(test)]
mod test {
    use super::find_common_in_vec;


    #[test]
    fn test_1() {
        //[1,2,3]                 [1,2,3]             [1,2,3]
        let vec_a = vec![1,2,3];
        let vec_b = vec![1,2,3];
        find_common_in_vec(vec_a, vec_b);
    }

    #[test]
    fn test_2() {
        //[4,5,6]                 [1,4,5]             [4,5]
        let vec_a = vec![4,5,6];
        let vec_b = vec![1,4,5];
        find_common_in_vec(vec_a, vec_b);
    }

    #[test]
    fn test_3() {
        // [3,4,5]                 [1,2,3]             [3]
        let vec_a = vec![3,4,5];
        let vec_b = vec![1,2,3];
        find_common_in_vec(vec_a, vec_b);
    }

    #[test]
    fn test_4() {
        //[4,5,6]                 [1,2,3]             []
        let vec_a = vec![4,5,6];
        let vec_b = vec![1,2,3];
        find_common_in_vec(vec_a, vec_b);
    }
}