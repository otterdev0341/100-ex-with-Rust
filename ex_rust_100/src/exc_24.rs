/*
    Write a  program that counts the number of elements in a vec with value greater than 3.

    You may assume that the vec only contains numbers.

    Print the final count.

    {Vec}               {Output}
    [1,-1,0,2,2,3]      0
    [1,2,3,4]           1
    [7,8,9,10]          4
    []                  0

*/

fn count_vec_that_gr_three(data: Vec<i32>) {
    
    let count = data.iter().filter(|&&x|x > 3 ).count();
    println!("{}",count);
}

#[cfg(test)]
mod test {
    use super::count_vec_that_gr_three;


    // [1,-1,0,2,2,3]      0
    #[test]
    fn test_1() {
        let data: Vec<i32> = vec![1, -1, 0, 2, 2, 3];
        count_vec_that_gr_three(data);
    }

    #[test]
    fn test_2() {
        //  [1,2,3,4]           1
        let data: Vec<i32> = vec![1,2,3,4];
        count_vec_that_gr_three(data);
    }

    #[test]
    fn test_3() {
        //  [7,8,9,10]          4
        let data: Vec<i32> = vec![7,8,9,10];
        count_vec_that_gr_three(data);
    }

    #[test]
    fn test_4() {
        // []                  0
        let data: Vec<i32> = vec![];
        count_vec_that_gr_three(data);
    }

}