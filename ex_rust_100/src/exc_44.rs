/*

Write a  program that takes the content of a HashMap and converts it into a vec of vecs.

Each nested vec should contain a key as the first element and its corresponding value as the second element.

Print the final vec of vecs.

Expected Output:
If this is the original dictionary:

product_info = {
	"description": "shoe",
	"price": 4.56,
	"colors": ["green", "blue", "red"],
}
The output should be:

[['description', 'shoe'], ['price', 4.56], ['colors', ['green', 'blue', 'red']]]

*/
enum Case {
    String,
    Float,
    VecString,
    Default,
}

pub trait StringConvertible: Sized {
    fn from_string(value: String) -> Self;
    fn to_string(&self) -> String;
}

impl StringConvertible for String {
    fn from_string(value: String) -> Self {
        value
    }

    fn to_string(&self) -> String {
        self.clone()
    }
}

pub trait CastFromFloat: Sized {
    fn cast_from_float(value: f64) -> Self;
}

impl CastFromFloat for f64 {
    fn cast_from_float(value: f64) -> Self {
        value
    }
}

use std::{any::{self, Any}, collections::{btree_map::Values, HashMap}, fmt::{Debug, Display}};

fn tranform_hash_to_vec<T>(data: HashMap<String, Box<dyn Any>>)
where 
    T : Any + Clone + Debug + Display + StringConvertible + CastFromFloat,
{
    let mut result_vec: Vec<T> = vec![];
    let mut fn_case: Case = Case::Default;
    for (key, val) in data.into_iter() {
        //temp valut to insert with final extraction
        let mut key_vec: Vec<String> = vec![];
        let mut value_vec_string: Vec<String> = vec![];
        let mut value_vec_f64: Vec<f64> = vec![];
        
        // key always be string cant insert in this process
        result_vec.push(T::to_string(key));

        //check what kind of value that we got
        // String || number || vector
        if let Some(string_val) = val.downcast_ref::<String>() {
            
            value_vec_string.push(string_val.to_owned());
            fn_case = Case::String

        } else if let Some(int_val) = val.downcast_ref::<f64>() {

            value_vec_f64.push(*int_val);
            fn_case = Case::Float

        } else if let Some(vec_val) = val.downcast_ref::<Vec<String>>() {
            
            value_vec_string.clear();
            for data in vec_val {
                value_vec_string.push(data.to_owned());
            }
            fn_case = Case::VecString
        } else {
            
        }
        match fn_case {
            Case::String => {
                result_vec.push(value_vec_string);
            },
            Case::Float => {
                result_vec.push(T::cast_from_float(value_vec_f64[0]));
            },
            Case::VecString => {
                println!("");
            },
            Case::Default => (),
        }
    }// end main for

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

        tranform_hash_to_vec(context);
    }
}