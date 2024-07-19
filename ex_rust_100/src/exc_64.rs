/*
Write a  program that checks if a number is prime or not.

If the number is prime, it should print "Prime".

If the number is not prime, it should print "Not prime".

A prime number is only divisible by 1 and by itself. It is not the product of two smaller natural numbers.
*/

fn is_it_prime(number: i32) -> bool {
    
    
    if number <= 1 {
        return false;
    }

    if number == 2 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    let limit = (number as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::is_it_prime;


    #[test]
    fn test_1() {
        // 4 false
        
        assert_eq!(false,is_it_prime(4));
    }

    #[test]
    fn test_2() {
        // 15 false
        assert_eq!(false,is_it_prime(15));
    }

    #[test]
    fn test_3() {
        // 1 false
        assert_eq!(false,is_it_prime(15));
    }

    #[test]
    fn test_4() {
        // 3 true
        assert_eq!(true,is_it_prime(3));
    }

    #[test]
    fn test_5() {
        // 2 true
        assert_eq!(true,is_it_prime(2));
    }

    #[test]
    fn test_6() {
        // 0 false
        assert_eq!(false,is_it_prime(0));
    }
}