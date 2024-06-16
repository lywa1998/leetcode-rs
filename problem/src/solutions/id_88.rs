struct Solution;
#[allow(clippy::ptr_arg)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        for idx in (0..(m + n) as usize).rev() {
            let n1 = if m > 0 {
                nums1[m as usize - 1]
            } else {
                i32::MIN
            };
            let n2 = if n > 0 {
                nums2[n as usize - 1]
            } else {
                i32::MIN
            };
            if n1 < n2 {
                nums1[idx] = n2;
                n -= 1;
            } else {
                nums1[idx] = n1;
                m -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let ans = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(nums1, ans);
    }

    #[test]
    fn example2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let ans = vec![1];
        assert_eq!(nums1, ans);
    }

    #[test]
    fn example3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let ans = vec![1];
        assert_eq!(nums1, ans);
    }
}
