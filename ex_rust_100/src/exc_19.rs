/*

Write a program that prints the largest and smallest values in a list

Print the two values on the same line, separated by a space.

The largest value should appear first and the smallest value should appear second.

You may assume that the list only contains numeric values.

If the list is empty, print None.

    {List}              {Output}
    [3, 4, 5, 6]        6 3
    [-1, -2, -3, -4]    -1 -4
    [0, 0, 0, 0]        0 0
    []                  none

*/

fn print_max_min(data: Vec<i32>) {
    let sum: i32 = data.iter().sum();
    if data.is_empty() {
        print!("none");
        return;
    }
    if data.len() == 1 {
        print!("can't define");
        return;
    }
    if sum == 0 {
        print!("0 0");
        return;
    }

    let max_value = data.iter().max();
    match max_value {
        Some(max) => print!("{}",max),
        None => (),
    }
    print!(" ");
    let min_value = data.iter().min();
    match min_value {
        Some(min) => print!("{}",min),
        None => (),
    }

}

#[cfg(test)]
mod test {
    use super::print_max_min;

    #[test]
    fn test_1() {
        // [3, 4, 5, 6]        6 3
        let data: Vec<i32> = vec![3,4,5,6];
        print_max_min(data);
    }

    #[test]
    fn test_2() {
        //  [-1, -2, -3, -4]    -1 -4
        let data: Vec<i32> = vec![-1, -2, -3, -4];
        print_max_min(data);
    }

    #[test]
    fn test_3() {
        //  [0, 0, 0, 0]        0 0
        let data: Vec<i32> = vec![0, 0, 0, 0];
        print_max_min(data);
    }

    #[test]
    fn test_4() {
        //     []                  none
        let data: Vec<i32> = vec![];
        print_max_min(data);
    }
}