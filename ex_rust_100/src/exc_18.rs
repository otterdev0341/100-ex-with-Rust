/*
    Write a  program that prints the elements of a vec on the same line.

    The elements should be separated only by a space (not by a comma).

    The output should not include the opening and closing square brackets [ ].
    
    {Vec}                  {Output}
    [3, 4,5, 6]             3 4 5 6
    ["a", "b", "c"]         a b c
*/

use core::fmt;


fn print_element<T>(data: Vec<T>) 
where
    T : fmt::Debug + std::fmt::Display,
{

    let length = data.len() - 1;
    let mut cursor = 0;
    if length > 0 {
        while cursor <= length {
            print!("{}",data[cursor]);
            if cursor < length {
                print!(" ");
            }
            cursor = cursor + 1;
        }
    }
    
}

#[cfg(test)]
mod test{
    use super::print_element;



    #[test]
    fn test_1() {
        // [3, 4, 5, 6]             3 4 5 6
        let context: Vec<_> = vec![3,4,5,6];
        print_element(context);
        
    }

    #[test]
    fn test_2() {
        // ["a", "b", "c"]         a b c
        let context: Vec<_> = vec!["a", "b", "c"];
        print_element(context);
    }

}