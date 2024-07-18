/*
Write a  program that prints if a given year was (or will) be a leap year.

Tip: A leap year is "a year, occurring once every four years, that has 366 days including February 29 as an intercalary day." (Definition by Oxford Languages).

This is how you can determine if a year is a leap year or not:

    if (year is not divisible by 4) then (it is a common year).

    else if (year is not divisible by 100) then (it is a leap year)

    else if (year is not divisible by 400) then (it is a common year)

    else (it is a leap year)

    {input}             {output}
    2025                No
    2033                No
    1836                Yes
    1912                Yes
*/

fn is_leap_year(year: i32) -> String {
    
    //COMMON year % 4 != 0  && year % 400 != 0 >>> println common year
    // LEAP year / 4
    let mut result: String = "".to_owned();
    if year % 4 != 0 {
        result = "No".to_owned();
    } else if year % 100 != 0 {
        result = "Yes".to_owned();
    } else if year % 400 != 0 {
        result = "No".to_owned();
    } else {
        result = "Yes".to_owned();
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::is_leap_year;


    #[test]
    fn test_1() {
        // 2025                No
        let year = 2025;
        let expect = "No".to_owned();
        assert_eq!(expect, is_leap_year(year));
    }

    #[test]
    fn test_2() {
        // 2033                No
        let year = 2033;
        let expect = "No".to_owned();
        assert_eq!(expect, is_leap_year(year));
    }

    #[test]
    fn test_3() {
        // 1836                Yes
        let year: i32 = 1836;
        let expect = "Yes".to_owned();
        assert_eq!(expect, is_leap_year(year));
    }

    #[test]
    fn test_4() {
        // 1912                Yes
        let year: i32 = 1912;
        let expect = "Yes".to_owned();
        assert_eq!(expect, is_leap_year(year));
    }
}