struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let bytes = s.as_bytes();
        let mut first_idx = [-1; 26];
        
        for (cur_idx, &byte) in bytes.iter().enumerate() {
            let byte_idx = (byte as u8 - b'a') as usize;
            let byte_first_idx = first_idx[byte_idx];
            if byte_first_idx == -1 {
                first_idx[byte_idx] = cur_idx as i32;
            } else if (cur_idx as i32 - byte_first_idx) != distance[byte_idx] + 1 {
                return false;
            }
        }
        
        return true;
    }
}