/*
    Write a program that multiplies all the items in a list by the value of the variable factor.

    The program must print the list as the output.

    The program should also allow multiplying the variable factor by a string in case the list contains strings.

    You may assume that the value of factor will be a positive integer.

    {List}              {factor}                    {Output}
    [3, 4, 5, 6]            2                      [6, 8 ,10, 12]
    ["a", "b", "c"]         3                   ["aaa", "bbb", "ccc"]
*/


use core::{any::{Any, TypeId}, fmt};

trait RepeatToString {
    fn toString(&self) -> String;
}
impl RepeatToString for String {
   
    fn toString(&self) -> String {
        self.clone()
    }
}

impl RepeatToString for usize {
   
    fn toString(&self) -> String {
        self.to_string()
    }
}


fn ampifly_thing<T>(list:&Vec<T> , factor: usize) -> Vec<String>
where
    T: Any + Clone + fmt::Debug + RepeatToString  ,
{
    
    let mut result: Vec<_> = vec![];
    let mut result_num: Vec<_> = vec![];
    let mut final_result: Vec<String> = vec![];

    if !list.is_empty() && factor > 1 {
        let is_string_vec = is_first_is_type_string(&list);
        // string vec case
        if is_string_vec {
            for data in list.iter() {
                let temp: String = data.toString().repeat(factor);
                result.push(temp);
            }
            final_result = result;
            
        }else {
            for num in list.iter() {
                let temp = num.toString();
                let convert_num: Result<usize, _> = str::parse(&temp);
                match convert_num {
                    Ok(number) => result_num.push((number * factor).to_string()),
                    Err(_) => (),
                }
            }
            final_result = result_num;
        }

    }
    return final_result;
 

}


fn is_first_is_type_string<T: Any>(vec: &Vec<T>) -> bool
{
    if let Some(first_element) = vec.first() {
        first_element.type_id() == TypeId::of::<String>()
    } else {
        false
    }
}



#[cfg(test)]
mod test {
    
    use crate::exc_17::ampifly_thing;

    #[test]
    fn test_1() {
        // [3, 4, 5, 6]            2                      [6, 8 ,10, 12]
        let data = vec![3,4,5,6];
        let expect: Vec<usize> = vec![6,8,10,12];
        let factor: usize = 2;
        let result = ampifly_thing(&data, factor);
        assert_eq!(
            expect.iter().map(|&num| num.to_string()).collect::<Vec<String>>(),result
        );
    }

    #[test]
    fn test_2() {
        // ["a", "b", "c"]         3                   ["aaa", "bbb", "ccc"]
        let data = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
        let expect = vec!["aaa", "bbb", "ccc"];
        let factor = 3;
        let result = ampifly_thing(&data, factor);
        assert_eq!(expect,result);
    }
}