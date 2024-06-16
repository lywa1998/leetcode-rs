#![allow(unused)]
struct Solution;

impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        salary.sort();
        let sum_salary = salary
            .iter()
            .enumerate()
            .filter(|(idx, _)| 0 < *idx && *idx < salary.len() - 1)
            .fold(0, |sum, (_, s)| sum + s);
        sum_salary as f64 / (salary.len() - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let salary = vec![4000, 3000, 1000, 2000];
        let ans = 2500.0f64;
        let res = Solution::average(salary);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_2() {
        let salary = vec![1000, 2000, 3000];
        let ans = 2000f64;
        let res = Solution::average(salary);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_3() {
        let salary = vec![6000, 5000, 4000, 3000, 2000, 1000];
        let ans = 3500f64;
        let res = Solution::average(salary);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_4() {
        let salary = vec![8000, 9000, 2000, 3000, 6000, 1000];
        let ans = 4750f64;
        let res = Solution::average(salary);
        assert_eq!(res, ans);
    }
}
