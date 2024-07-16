/*
Write a  program that counts the frequency of each value in a HashMap.

The program should create a new HashMap to map each value in the original HashMap to its frequency (how many times it occurs).

If the HashMap is empty, print an empty HashMap.

 my_hash = {
	"a": 4,
	"b": 4,
	"c": 2,
	"d": 7,
	"e": 4,
	"f": 2,
	"g": 7,
	"h": 7
 }

 The output should be:
    
    freq_dict = {
	4: 3 
	2: 2
	7: 3
}

*/

use std::{any::Any, collections::HashMap, fmt::{Debug, Display}, hash::Hash};


fn print_freequency_in_hashmap<T>(data: HashMap<T,usize>)
where
    T : Any + Hash + Eq + Clone + Display + Debug,
{   // create tuple vec
    let mut tup_vec: Vec<(usize,usize)> = vec![];
    //create only value from hashmap
    let vec_val: Vec<_> = data.into_values().collect();
    // tranform into tuple vec
    for item in vec_val.iter() {
        tup_vec.push((*item,1));
    }

    //try add into hash to print
    let mut result: HashMap<usize,usize> = HashMap::new();

    for (val, num) in tup_vec.iter() {
        if result.contains_key(val) {
            if let Some(x) = result.get_mut(val) {
                *x = *x + 1
            }
        } else {
            result.insert(*val, *num);
        }
    }
    //loop to display
    for item in result.iter(){
        println!("{:?}: {:?}",item.0, item.1);

    }
}

    
    


    

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::print_freequency_in_hashmap;


    #[test]
    fn test_1() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("a".to_owned(), 4);
        data.insert("b".to_owned(), 4);
        data.insert("c".to_owned(), 2);
        data.insert("d".to_owned(), 7);
        data.insert("e".to_owned(), 4);
        data.insert("f".to_owned(), 2);
        data.insert("g".to_owned(), 7);
        data.insert("h".to_owned(), 7);
        print_freequency_in_hashmap(data);
    }
}

