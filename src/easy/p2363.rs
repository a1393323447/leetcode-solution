struct Solution;

impl Solution {
    pub fn merge_similar_items(
        mut items1: Vec<Vec<i32>>,
        mut items2: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        items1.sort_by(|a, b| a[0].cmp(&b[0]));
        items2.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut res: Vec<Vec<i32>> = vec![];
        let mut p1 = 0;
        let mut p2 = 0;

        while p1 < items1.len() || p2 < items2.len() {
            let new_item;
            if p1 >= items1.len() {
                new_item = items2[p2].clone();
                p2 += 1;
            } else if p2 >= items2.len() {
                new_item = items1[p1].clone();
                p1 += 1;
            } else if items1[p1][0] < items2[p2][0] {
                new_item = items1[p1].clone();
                p1 += 1;
            } else {
                new_item = items2[p2].clone();
                p2 += 1;
            }

            if let Some(v) = res.last_mut() {
                if new_item[0] == v[0] {
                    v[1] += new_item[1];
                    continue;
                }
            }

            res.push(new_item);
        }

        res
    }
}
