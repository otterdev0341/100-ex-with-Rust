/*

Write a  program that prints a descriptive message indicating if each character in a string is a vowel or a consonant. 
For example: "<char> is a <consonant | vowel>"

If the character is a special character, number, or symbol, print "<char> is not a letter".

If the string is empty, print "Empty String".

🔹 Expected Output:
If the string is:

"Score: 36"

The expected output would be:

s is a consonant
c is a consonant
o is a vowel
r is a consonant
e is a vowel
: is not a letter
  is not a letter
3 is not a letter
6 is not a letter
You can customize the message displayed.
*/

fn check_kind_of_string(context: String) {

    let vec_char: Vec<_> = context.chars().collect();
    let vec_vovel: Vec<_> = vec!["a".to_owned(),"e".to_owned(),"i".to_owned(),"o".to_owned(),"u".to_owned()];

    for ch in vec_char.iter() {
        if ch.is_alphabetic() {
            if vec_vovel.contains(&ch.to_string()) {
                println!("{} is a vowel",ch);
            } else {
                println!("{} is a consonant",ch);
            }
        } else {
            println!("{} is not a letter",ch);
        }
    }

}

#[cfg(test)]
mod test {
    use super::check_kind_of_string;


    #[test]
    fn test_1() {
        let context = "score: 36".to_owned();
        check_kind_of_string(context);
    }
}