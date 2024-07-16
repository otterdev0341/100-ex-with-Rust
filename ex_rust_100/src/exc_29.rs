/*

Write a program that prints the second small value in a vec.

If the vec is empty or only has one element, print None.

    {input}             {output}
    [1,2,3,4]           2
    [1,3]               3
    [2]                 None
    []                  None
*/
use std::any::Any;

fn find_second_small_in_vec<T>(data: Vec<T>)
where
    T : Any + PartialOrd + Ord + std::fmt::Display + std::fmt::Debug + Clone,
{
    if data.is_empty() || data.len() == 1 {
        println!("None");
        return;
    }

    let mut clone_data: Vec<_> = data.iter().cloned().collect();
    clone_data.sort();

    println!("{}",clone_data[1]);

}

#[cfg(test)]
mod test {
    use super::find_second_small_in_vec;


    #[test]
    fn test_1() {
        //[1,2,3,4]           2
        let data = vec![1,2,3,4];
        find_second_small_in_vec(data);
    }

    #[test]
    fn test_2() {
        // [1,3]               3
        let data = vec![1,3];
        find_second_small_in_vec(data);
    }

    #[test]
    fn test_3() {
        // [2]                 None
        let data = vec![2];
        find_second_small_in_vec(data);
    }

    #[test]
    fn test_4() {
        // []                  None
        let data: Vec<i32> = vec![];
        find_second_small_in_vec(data);
    }
}