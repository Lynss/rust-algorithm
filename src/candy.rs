use std::cmp::max;

///my solution
//pub fn candy(ratings: Vec<i32>) -> i32 {
//    let range = ratings.len();
//    let mut result = vec![1; range];
//    fn change_before(index: usize, result_value: i32, result: &mut Vec<i32>, ratings: &Vec<i32>) {
//        if result[index - 1] <= result_value {
//            let new = result_value + 1;
//            result[index - 1] = new;
//            if index >= 2 && ratings[index - 1] < ratings[index - 2] {
//                change_before(index - 1, new, result, ratings);
//            }
//        };
//    };
//    (1..range).zip(&ratings[1..]).for_each(|(index, &value)| {
//        let before = index - 1;
//        let before_value = ratings[before];
//        if value > before_value {
//            let before_result_value = result[before] + 1;
//            result[index] = before_result_value;
//        } else if value != before_value{
//            change_before(index, 1, &mut result, &ratings);
//        };
//    });
//    result.iter().sum()
//}

/// a easier solution
pub fn candy(ratings: Vec<i32>) -> i32 {
    let range = ratings.len();
    let mut result = vec![1; range];
    (1..range).for_each(|index| {
        if ratings[index] > ratings[index - 1] {
            result[index] = result[index - 1] + 1;
        }
    });
    (0..range - 1).rev().for_each(|index| {
        if ratings[index] > ratings[index + 1] {
            result[index] = max(result[index + 1] + 1, result[index])
        }
    });
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_candy() {
        let ratings_1 = vec![1, 0, 2];
        let ratings_2 = vec![1, 2, 2];
        assert_eq!(candy(ratings_1), 5);
        assert_eq!(candy(ratings_2), 4);
    }
}
