/*
Write a program that prints (as a vec) the elements of vecA that are not in vecB as a vec.

If the vec have the same elements, print an empty list.

If vecA is an empty list, print an empty Vec.

    {vecA}                  {vecB}              {output}
    [1,2,3,4]               [1,2]               [3,4]
    [1,2,3,4]               [1,2,3]             [4]
    [1,2,3,4]               [1,2,3,4]           []
    []                      [1,3]               []

*/

use std::fmt::{Debug, Display};
use std::any::Any;

fn find_diff<T>(vec_a: Vec<T>, vec_b: Vec<T>)
where
    T : Any + PartialEq + Clone + Display + Debug,
{
    // case empty vec
    if vec_a.is_empty() {
        println!("[]");
        return;
    }

    let vec_diff: Vec<_> = vec_a.iter().filter(|&item| !vec_b.contains(item)).cloned().collect();
    if vec_diff.is_empty() {
        println!("[]");
        return;
    }
    println!("{:?}",vec_diff);
}

#[cfg(test)]
mod test {
    use super::find_diff;


    #[test]
    fn test_1() {
        // [1,2,3,4]               [1,2]               [3,4]
        let vec_a = vec![1,2,3,4];
        let vec_b = vec![1,2];
        find_diff(vec_a, vec_b);
    }

    #[test]
    fn test_2() {
        // [1,2,3,4]               [1,2,3]             [4]
        let vec_a = vec![1,2,3,4];
        let vec_b = vec![1,2,3];
        find_diff(vec_a, vec_b);
    }

    #[test]
    fn test_3() {
        // [1,2,3,4]               [1,2,3,4]           []
        let vec_a = vec![1,2,3,4];
        let vec_b = vec![1,2,3,4];
        find_diff(vec_a, vec_b);
    }

        #[test]
    fn test_4() {
        // []                      [1,3]               []
        let vec_a: Vec<i32> = vec![];
        let vec_b = vec![1,3];
        find_diff(vec_a, vec_b);
    }
}

