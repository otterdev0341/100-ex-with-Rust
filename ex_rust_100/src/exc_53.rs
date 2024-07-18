/*
Write a  program that prints the positive and negative solutions (roots) for a quadratic equation.

If the equation only has one solution, print the solution as the output.

If it has two solutions, print the negative one first and the positive one second on the same line.

If the equation has no real solutions, print "Complex Roots".

You can determine the number of solutions with the discriminant (the result of b^2 - 4ac in the formula below).

- If it's negative, the equation has no real solutions (only complex roots).

- If it's 0, there is only one solution.

- If it's positive, there are two real solutions.
    {a,b,c}             {output}
    1,2,1               -1
    2,5,-3              -3 0.5
    3,4,5               "Complex Roots"
*/

fn quadratic_eq(a: f64, b: f64, c: f64) {
    
    let discriminant = b * b - 4.0 * a * c; 

    if discriminant < 0.0 {
        println!("Complex Roots");
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("{}", root);
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b - sqrt_discriminant) / (2.0 * a);
        let root2 = (-b + sqrt_discriminant) / (2.0 * a);
        println!("{} {}", root1, root2);
    }
}


#[cfg(test)]
mod test {
    use super::quadratic_eq;


    #[test]
    fn test_1() {
        // 1,2,1               -1
        let a = 1.0;
        let b = 2.0;
        let c = 1.0;

        quadratic_eq(a, b, c);
    }

    #[test]
    fn test_2() {
        //   2,5,-3              -3 0.5
        let a = 2.0;
        let b = 5.0;
        let c = -3.0;

        quadratic_eq(a, b, c);
    }

    #[test]
    fn test_3() {
        //3,4,5               "Complex Roots"
        let a = 3.0;
        let b = 4.0;
        let c = 5.0;
        quadratic_eq(a, b, c);
    }
}

