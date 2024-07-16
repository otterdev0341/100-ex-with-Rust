/*
Write a  program that creates and print a dictionary that maps each element in a vec to its corresponding frequency (how many times it occurs in the vec).

The test should be case-sensitive. Therefore, "A" should not be considered the same element as "a".
    {list}                          {output}
    ["a","a","b","c","a","b"]       {"a" : 3, "b": 2, "c" : 1}
    [1,2,3,4,3,2,1,2]               [1:2, 2: 3, 3:2, 4:1]
*/

use std::{any::Any, collections::HashMap};

fn show_frequency<T>(data: Vec<T>)
where
    T : Any + Clone + std::fmt::Debug + std::fmt::Display + std::hash::Hash + Eq,
{
    let mut temp_data: Vec<(T, u32)> = vec![];
    let mut result: HashMap<T, u32> = HashMap::new();

    // create temp vec[tuple()] that contain (value, 1) bec value can dupicate in tuple
    for item in data.iter() {
        temp_data.push((item.clone(),1));
    }
    // try append into hashmap
    // if have *val incress by 1
    // else insert into hashmap
    for (val, count) in temp_data.into_iter() {
        if let Some(have_it) = result.get_mut(&val) {
            *have_it = *have_it + count;
        } else {
            result.insert(val, count);
        }
    }
    

    println!("{:?}",result);
}

#[cfg(test)]
mod test {
    use super::show_frequency;


    #[test]
    fn test_1() {
        //["a","a","b","c","a","b"]       {"a" : 3, "b": 2, "c" : 1}
        let data = vec!["a".to_owned(),"a".to_owned(),"b".to_owned(),"c".to_owned(),"a".to_owned(),"b".to_owned()];
        show_frequency(data);
    }

    #[test]
    fn test_2() {
        // [1,2,3,4,3,2,1,2]               [1:2, 2: 3, 3:2, 4:1]
        let data = vec![1,2,3,4,3,2,1,2];
        show_frequency(data);
    }
}