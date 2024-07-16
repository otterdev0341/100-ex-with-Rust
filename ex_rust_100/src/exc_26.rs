/*
    Write a program that calculates the distance between two 3D points.

    The points are represented by two lists with three elements. The first element is the x-coordinate. The second element is the y-coordinate. The third element is the z-coordinate.

    [pointA]            [pointB]            [output]
    [1,2,3]             [1,2,3]                 0
    [3,4,5]             [1,3,5]                 2.23607
    [-3,4-5]            [2,0,-4]                6.48074

*/
use std::any::Any;

fn cal_3d_distance(vec_a: Vec<f64>, vec_b: Vec<f64>) -> f64 
{

    if vec_a.len() != 3 || vec_b.len() != 3 {
        println!("one of vector not contain 3 elements");
        return 0.0;
    }
    let x = vec_a[0] - vec_b[0];
    let y = vec_a[1] - vec_b[1];
    let z = vec_a[2] - vec_b[2];
    
    (x * x + y * y + z * z).sqrt()


}

#[cfg(test)]
mod test {
    use crate::exc_26::cal_3d_distance;


    #[test]
    fn test_1() {
        // [1,2,3]             [1,2,3]                 0
        let vec_a: Vec<f64> = vec![1.0,2.0,3.0];
        let vec_b: Vec<f64> = vec![1.0,2.0,3.0];
        let expect = 0.0;
        assert_eq!(expect,cal_3d_distance(vec_a, vec_b));
    }

    #[test]
    fn test_2() {
        // [3,4,5]             [1,3,5]                 2.23606797749979
        let vec_a: Vec<f64> = vec![3.0, 4.0, 5.0];
        let vec_b: Vec<f64> = vec![1.0,3.0,5.0];
        let expect = 2.23606797749979;
        assert_eq!(expect, cal_3d_distance(vec_a, vec_b));
    }

    #[test]
    fn test_3() {
        // [-3,4-5]            [2,0,-4]                6.48074069840786
        let vec_a: Vec<f64> = vec![-3.0,4.0,-5.0];
        let vec_b: Vec<f64> = vec![2.0, 0.0, -4.0];
        let expect = 6.48074069840786;
        assert_eq!(expect, cal_3d_distance(vec_a, vec_b));
    }
}

