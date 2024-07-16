/*
Write a  program that merges two dictionaries and prints the resulting hashmap.

"Merging" the dictionaries involves making a new hashmap with the key-value pairs in both dictionaries.

🔹 Expected Output:
From these two dictionaries:

my_dict1 = {"a": 1, "b": 2, "c": 3}
my_dict2 = {"c": 4, "d": 6, "e": 8}
The output should be:

final_dict = {'a': 1, 'b': 2, 'c': 4, 'd': 6, 'e': 8}
🔸 Hints:
Notice that the key-value pairs that share the same key on both dictionaries are not repeated. They are updated with the value of the second hashmap.

There is a  operator that you can use to merge two or more dictionaries.

*/

use std::{collections::HashMap, fmt::{Debug, Display}, hash::Hash};
use std::any::Any;
fn merge_hash<T>(hash_1: HashMap<T,usize>, hash_2: HashMap<T,usize>)
where 
    T : Any + Clone + Display + Debug + Hash + Eq,
{
    
    let mut merge_hash: HashMap<T,usize> = HashMap::new();
    let mut clone_hash_2 = hash_2.clone();
    merge_hash = hash_1.clone();

    for (key, val) in clone_hash_2.iter_mut() {
        if merge_hash.contains_key(key) {
            if let Some(update) = merge_hash.get_mut(key) {
                *update = *val;
            }
        } else {
            merge_hash.insert(key.clone(), *val);
        }
    }

    println!("{:?}",merge_hash);

}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::merge_hash;


    #[test]
    fn test_1() {
        let mut hash_1: HashMap<String, usize> = HashMap::new();
        let mut hash_2: HashMap<String, usize> = HashMap::new();

        hash_1.insert("a".to_owned(), 1);
        hash_1.insert("b".to_owned(), 2);
        hash_1.insert("c".to_owned(), 3);

        hash_2.insert("c".to_owned(), 4);
        hash_2.insert("d".to_owned(), 6);
        hash_2.insert("e".to_owned(), 8);

        merge_hash(hash_1, hash_2);


    }
}
