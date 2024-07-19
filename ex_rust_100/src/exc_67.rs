/*

Write a  program that prints a string reversed using a loop.

All the characters must be on the same line in reverse order.

        {String}                {Output}
        "Hello"                 "olleH"
        "World"                 "dlroW"
        "Python"                "nohtyP"

*/

fn reverst_this_word(context: String) -> String {
   let reversed: String = context.chars().rev().collect();
   reversed
}


#[cfg(test)]
mod test {
    use crate::exc_67::reverst_this_word;


    #[test]
    fn test_1() {
        // "Hello"                 "olleH"
        let expect = "olleH".to_owned();
        let context = "Hello".to_owned();
        assert_eq!(expect, reverst_this_word(context));
    }


    #[test]
    fn test_2() {
        // "Hello"                 "olleH"
        let expect = "dlroW".to_owned();
        let context = "World".to_owned();
        assert_eq!(expect, reverst_this_word(context));
    }

    #[test]
    fn test_3() {
         // "Hello"                 "olleH"
         let expect = "nohtyP".to_owned();
         let context = "Python".to_owned();
         assert_eq!(expect, reverst_this_word(context));
    }
}


