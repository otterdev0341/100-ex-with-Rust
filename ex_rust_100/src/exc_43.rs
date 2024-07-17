/*
Write a  program that sorts (in ascending order) the vec contained as values in a HashMap.

The HashMap contains key-value pairs that match strings to vec. You need to sort these vec.

The vec have to be mutated (changed).

🔹 Expected Output:
If this is the HashMap:

my_dict = {
	"a": [4, 3, 2],
	"b": [5, 3, 7],
	"c": [1, 9, 10],
	"d": [3, 4, 1]
}
The final output should be:

my_dict = {
	"a": [2, 3, 4],
	"b": [3, 5, 7],
	"c": [1, 9, 10],
	"d": [1, 3, 4]
}
Notice how all the vec are now sorted in ascending order.
*/

use std::collections::HashMap;

use itertools::Itertools;

fn do_inner_sort(data: &mut HashMap<String, Vec<i32>>) {
    
    
    for (_ky, val) in data.iter_mut() {
        val.sort();
    }
    println!("{:?}",data);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::do_inner_sort;


    #[test]
    fn test_1() {
        let mut data: HashMap<String, Vec<i32>> = HashMap::new();
        data.insert("a".to_owned(), vec![4,3,2]);
        data.insert("b".to_owned(), vec![5,3,7]);
        data.insert("c".to_owned(), vec![1,9,10]);
        data.insert("d".to_owned(), vec![3,4,1]);
        do_inner_sort(&mut data);
    }

}