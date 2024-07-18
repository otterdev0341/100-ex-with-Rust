/*
Write a  program that prints the number of days in a given month.

The value of the variable month is the name of the month with the first letter capitalized.

Do not consider leap years for the number of days in February.

You can add a customized message. For example: "<month> has: <num_days> days."

*/


fn print_day_in_month(month: String) {
    
    if month.is_empty() {
        println!("Please give a correct month.");
        return;
    }
    
    let case = to_capital(month);
    let days_31: Vec<_> = vec![
        "January".to_owned(),
        "March".to_owned(),
        "May".to_owned(),
        "July".to_owned(),
        "August".to_owned(),
        "October".to_owned(),
        "December".to_owned()];
    let days_30: Vec<_> = vec![
        "April".to_owned(),
        "June".to_owned(),
        "September".to_owned(),
        "November".to_owned(),];
    let day_28: Vec<_> = vec!["February".to_owned()];
    if days_31.contains(&case) {
        println!("{:?} has 31 days.",case);
        return;

    } else if days_30.contains(&case) {
        println!("{:?} has 30 days.",case);
        return;
    } else if day_28.contains(&case) {
        println!("{:?} has 28 days.",case);
        return;
    } {
        println!("invalid please try again");
        return;
    }

}


fn to_capital(context: String) -> String {
    
    let trim_context = context.trim();
    let mut chars = trim_context.chars();

    let first_upper = chars.next().map(|c| c.to_uppercase().collect::<String>());
    let rest: String = chars.flat_map(|c| c.to_lowercase()).collect();

    let result = match first_upper {
        Some(first) => first + &rest,
        None => rest,
    };

    return result;


    
}

#[cfg(test)]
mod test {
    use super::print_day_in_month;


    #[test]
    fn test_1() {
        let list_month: Vec<_> = vec![
            "January".to_owned(),
            "February".to_owned(),
            "March".to_owned(),
            "April".to_owned(),
            "May".to_owned(),
            "June".to_owned(),
            "July".to_owned(),
            "August".to_owned(),
            "September".to_owned(),
            "October".to_owned(),
            "November".to_owned(),
            "December".to_owned()
        ];

        for x in list_month{
            print_day_in_month(x);
        }
    }
}