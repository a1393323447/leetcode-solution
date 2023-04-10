struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32, ay1: i32, 
        ax2: i32, ay2: i32, 
        bx1: i32, by1: i32, 
        bx2: i32, by2: i32
    ) -> i32 {
        let aw = ax2 - ax1;
        let ah = ay2 - ay1;

        let bw = bx2 - bx1;
        let bh = by2 - by1;

        let total = aw * ah + bw * bh;
        let intersect = 
            Solution::intersect((ax1, ax2), (bx1, bx2)) *
            Solution::intersect((ay1, ay2), (by1, by2));
        
        total - intersect
    }

    fn intersect(
        (s1, e1): (i32, i32),
        (s2, e2): (i32, i32),
    ) -> i32 {
        let smax = s1.max(s2);
        let emin = e1.min(e2);
        return (emin - smax).max(0);
    }
}