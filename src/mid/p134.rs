struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut i = 0;
        while i < len {
            let mut count = 0;
            let mut j;
            let mut gas_sum = 0;
            let mut cost_sum: i32 = 0;
            while count < len {
                j = (count + i) % len;
                gas_sum = gas_sum + gas[j];
                cost_sum = cost_sum + cost[j];
                if gas_sum >= cost_sum {
                    count = count + 1;
                } else {
                    break;
                }
            }
            if count == len {
                return i as i32;
            } else {
                i = i + count + 1;
            }
        }
        return -1;
    }
}
