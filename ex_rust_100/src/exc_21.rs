use core::fmt;
/*
    Write a program that prints the elements of a list followed their corresponding indices.

    Each element and its index must be on the same line separated by a space.

    If the list is empty, print "Empty List".
    
    {vec}                       {output}
    [1,2,3,4]                      1 0
                                   2 1
                                   3 2
                                   4 3
    
    ["a", "b", "c"]                a 0
                                   b 1
                                   c 2

    []                             "Empty List"
*/
use std::any::Any;

fn show_val_index<T>(data: Vec<T>)
where
    T : Any + fmt::Display,
{
    if data.is_empty() {
        println!("Empty List");
        return;
    }
    for (x, id) in data.iter().enumerate() {
        println!("{} {}",id,x);
    }
}

#[cfg(test)]
mod test{
    use super::show_val_index;
 

    #[test]
    fn test_1() {

        // [1,2,3,4]                  1 0
        //                            2 1
        //                            3 2
        //                            4 3
        let data = vec![1,2,3,4];
        show_val_index(data);
    }

    #[test]
    fn test_2() {
        // ["a", "b", "c"]                a 0
        //                                b 1
        //                                c 2
        let data = vec!["a", "b", "c"];
        show_val_index(data);
    }

    #[test]
    fn test_3() {
        // []                             "Empty List"
        let data: Vec<i32> = vec![];
        show_val_index(data);
    }

}
