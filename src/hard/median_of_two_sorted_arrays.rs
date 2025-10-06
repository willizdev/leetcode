struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i = 0;
        let mut j = 0;

        let m = nums1.len();
        let n = nums2.len();

        let mut k = 0;
        let mut l = 0;

        for _ in 0..=((m + n) / 2) {
            l = k;
            if i < m && j < n {
                if nums1[i] < nums2[j] {
                    k = nums1[i];
                    i += 1;
                } else {
                    k = nums2[j];
                    j += 1;
                }
            } else if i < m {
                k = nums1[i];
                i += 1;
            } else {
                k = nums2[j];
                j += 1;
            }
        }

        if (m + n) % 2 == 0 {
            return (k + l) as f64 / 2.0;
        }

        k as f64
    }
}