/*
📌 Description:
Write a Python program that adds a new key-value pair to a hashmap only if the key doesn't exist already.

If the key-value pair exists in the hashmap, do not update the existing value. The hashmap should not be modified in this case.

Store the new key in the new_key variable and the new value in the new_value variable.

Print the final value of the hashmap.

🔹 Expected Output:
Example 1: New Pair Added

Initial hashmap:

{"January": 45, "February": 56, "March": 67}
New Key-Value Pair:

"April": 67
Output:

{"January": 45, "February": 56, "March": 67, "April": 67}
Example 2: No Change

Initial hashmap:

{"January": 45, "February": 56, "March": 67}
New Key-Value Pair:

# The key already exists in the hashmap
"January": 67
Output:

{"January": 45, "February": 56, "March": 67}
*/

use std::{any::Any, collections::HashMap, fmt::{Debug, Display}, hash::Hash};

fn try_add_into_hash<T>(hm: HashMap<T,usize>, ky: T, val: usize)
where
    T : Any + Eq + Hash + Display + Debug + Clone,
{
    let mut result = hm.clone();
    if result.contains_key(&ky) {
        println!("\t\tcan't perform operation. The key already have into hashmap");
        println!("{:?}",result);
    } else {
        result.insert(ky, val);
        println!("Already add into hashmap");
        println!("{:?}",result);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::try_add_into_hash;


    #[test]
    fn test_1() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("January".to_owned(), 45);
        data.insert("Febuary".to_owned(), 56);
        data.insert("March".to_owned(), 67);
        try_add_into_hash(data, "April".to_owned(), 67);

    }

    #[test]
    fn test_2() {
        let mut data: HashMap<String, usize> = HashMap::new();
        data.insert("January".to_owned(), 45);
        data.insert("Febuary".to_owned(), 56);
        data.insert("March".to_owned(), 67);
        try_add_into_hash(data, "January".to_owned(), 67);
    }
}