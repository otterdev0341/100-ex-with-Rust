/*

Write a  program that prints the corresponding season based on the value of the variable season_num.

The possible values of season_num are: 1 for Spring, 2 for Summer, 3 for Fall, 4 for Winter.

If the value of season_num is neither one of these values, print "Please enter a valid number".


*/

fn season_based(num: i32) {

    match num {
        1 => println!("Spring"),
        2 => println!("Summer"),
        3 => println!("Fall"),
        4 => println!("Winter"),
        _ => println!("Please enter a valid number"),
    }

}

#[cfg(test)]
mod test {
    use super::season_based;


    #[test]
    fn test_1() {
        for x in 1..=5 {
            season_based(x);
        }
    }
}