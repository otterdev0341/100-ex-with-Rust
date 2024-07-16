/*
Write a Python program that generates and prints all the possible permutations of a vec.

A permutation is a possible arrangement of the elements of the vec. For example, [2, 1, 3] is a permutation of [1, 2, 3].

Print each permutation as a vec on a separate line. You can print them as vecs or tuples.

Include the vec itself as a permutation.
*/

use std::{any::Any, fmt::{Debug, Display}};
use itertools::Itertools; 




fn print_permutation_vec<T>(data: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + Debug + Ord,
{
    
    if data.len() == 1 {
        vec![data]
        
    } else {
        let mut output: Vec<Vec<T>> = vec![];

        let mut unique_items = data.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = data.clone();

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in print_permutation_vec(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }

    
}

#[cfg(test)]
mod test {
    use super::print_permutation_vec;

    #[test]
    fn test_1() {
        // [1,2,3] -> [1,2,3] \n [1,3,2] \n [2,1,3] \n [2,3,1] \n [3,1,2] \n [3,2,1]
        let data = vec![1, 2, 3];
        let result = print_permutation_vec(data);
        for x in result.into_iter() {
            println!("{:?}",x);
        }
    }
}