/*

Write a  program that takes the content of a HashMap and converts it into a vec of vecs.

Each nested vec should contain a key as the first element and its corresponding value as the second element.

Print the final vec of vecs.

*/

use std::{any::{self, Any}, collections::{btree_map::Values, HashMap}, fmt::{Debug, Display}};

fn tranform_hash_to_vec<T>(data: HashMap<String, Box<dyn Any>>)
where 
    T : Any + Clone + Debug + Display + ToString,
{
    let mut result_vec: Vec<T> = vec![];

    for (key, val) in data.into_iter() {
        //temp valut to insert with final extraction
        let mut key_vec: Vec<String> = vec![];
        let mut value_vec: Vec<T> = vec![];
        // key always be string cant insert in this process
        key_vec.push(String::from(key));

        //check what kind of value that we got
        // String || number || vector
        if let Some(string_val) = val.downcast_ref::<String>() {
            value_vec.push(string_val.to_owned());
        } else if let Some(int_val) = val.downcast_ref::<f64>() {
            
        } else if let Some(vec_val) = val.downcast_ref::<Vec<String>>() {
            
        } else {
            
        }
        
    }

    println!("{:?}",result_vec);
}

#[cfg(test)]
mod test {
    use std::{any::Any, collections::HashMap, fmt::{Debug, Display}};
    use super::tranform_hash_to_vec;

    #[test]
    fn test_1(){
        let mut context: HashMap<String,Box<dyn Any>> = HashMap::new();
        context.insert("description".to_owned(), Box::new("shoe".to_owned()));
        context.insert("price".to_owned(), Box::new(4.56));
        context.insert("color".to_owned(), Box::new(vec!["green".to_owned(),"blue".to_owned(),"red".to_owned()]));

        tranform_hash_to_vec::<String>(context);
    }
}