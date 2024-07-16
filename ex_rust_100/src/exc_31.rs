/*
Write a  program that prints a "flattened" version of a vec that contains nested vecs.

"Flattened" means that all the elements in the nested vecs should be added to a main vec such that it doesn't contain any nested vecs, just the individual.

The vec could contain other elements that are not vecs or other sequences, so you must check the type of each element and act appropriately.

If the vec is empty, print an empty vec.

    {input}                                             {output}
    [[1,2,3],[4,5,6],[7,8,9]]                       [1,2,3,4,5,6,7,8,9]
    [1,2,3,4,5,6[7,8,9]]                            [1,2,3,4,5,6,7,8,9]
    [["a","b","c"],["d","e","f"],["g","h","i"]]     ["a","b","c","d","e","f","g","h","i"]

*/
use std::{any::Any, fmt::{Debug, Display}};

fn flattened_nested_vec<T>(data: Vec<impl IntoIterator<Item = T>>)
where
    T : Any + Debug + Display+ Clone,
{
    let result: Vec<T> = data.into_iter()
                            .flat_map(|inner_vec| inner_vec.into_iter())
                            .collect();
    
    println!("{:?}",result);
}

#[cfg(test)]
mod test {
    use super::flattened_nested_vec;


    #[test]
    fn test_1() {
        //[[1,2,3],[4,5,6],[7,8,9]]               [1,2,3,4,5,6,7,8,9]
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        flattened_nested_vec(data);
    }

    #[test]
    fn test_2() {
       // [1,2,3,4,5,6[7,8,9]]                    [1,2,3,4,5,6,7,8,9]
    //    let data = vec![1,2,3,4,5, vec![7,8,9]];
        
    }

    #[test]
    fn test_3() {
        // [["a","b","c"],["d","e","f"],["g","h","i"]]     ["a","b","c","d","e","f","g","h","i"]
        let data = vec![
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            vec!["d".to_owned(), "e".to_owned(), "f".to_owned()],
            vec!["g".to_owned(), "h".to_owned(), "i".to_owned()]];
        flattened_nested_vec(data);
    }
}