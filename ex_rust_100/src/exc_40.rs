/*
Write a  program that creates a hashmap from the values contained in nested vec.

Each nested list has this format [value1, value2].

value1 should be the key in the hashmap and value2 should be its corresponding value.

If there are no nested vec, print an empty hashmap.

If this is the list that contains nested lists:

[["a", 1], ["b", 2], ["c", 3], ["d", 4]]
The result should be:

{"a": 1, "b": 2, "c": 3, "d": 4}
*/

use std::{any::Any, collections::HashMap, fmt::{Debug, Display}, hash::Hash, str::FromStr};

fn vec_to_hash<T>(data: Vec<Vec<T>>)
where
    T : Any + Display + Debug + Hash + Eq + Clone + FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut result_hash: HashMap<T,_> = HashMap::new();
    for item in data.into_iter() {
        
        let key = item[0].clone();
        let value = item[1].clone().to_string();
        let cast_to_usize: Result<usize,_> = value.parse();
        match cast_to_usize {
            Ok(thing) => {
                result_hash.insert(key, thing);
            },
            Err(e) => {
                println!("Error {:?}",e);
            }
        }
        
        

    }
    println!("{:?}",result_hash);
}

#[cfg(test)]
mod test {
    use super::vec_to_hash;


    #[test]
    fn test_1() {
        let data = vec![ 
            vec!["a".to_owned(),1.to_string()],
            vec!["b".to_owned(),2.to_string()],
            vec!["c".to_owned(),3.to_string()],
            vec!["d".to_owned(),4.to_string()]
        ];

        vec_to_hash(data);
    }
}