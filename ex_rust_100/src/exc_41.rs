/*
Write a Python program that prints the largest of the values in a HashMap.

You may assume that all the values in the HashMap are either vec or tuples.

{excpect}
If this is the dictionary:

my_dict = {
	"a": [1, 2, 3],
	"b": [4, 0, -4],
	"c": [3, 5, 9],
	"d": [45, 12, 100]
}
This should be the output: 157

    
*/

use std::{collections::HashMap, fmt::{Debug, Display}};

fn largest_value<T>(data: HashMap<T,Vec<i32>> )
where 
    T : Display + Debug + Clone,
{
    let mut hight_sum = 0;
    for (_key, val) in data.into_iter() {
        let sum_val: i32 = val.iter().sum();
        if sum_val > hight_sum {
            hight_sum = sum_val;
        }
    }
    println!("{:?}",hight_sum);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::largest_value;

    
    #[test]
    fn test_1() {
        let mut data: HashMap<String,Vec<_>> = HashMap::new();
        data.insert("a".to_owned(), vec![1,2,3]);
        data.insert("b".to_owned(), vec![4,0,-4]);
        data.insert("c".to_owned(), vec![3,5,9]);
        data.insert("d".to_owned(), vec![45,12,100]);

        largest_value(data);

    }
    
}
