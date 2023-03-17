struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut vis = vec![false;n];
        vis[0] = true;
        Solution::step(0, &mut vis, &rooms);
        vis.into_iter().all(|b| b)
    }

    fn step(
        node: usize,
        vis: &mut [bool],
        rooms: &Vec<Vec<i32>>,
    ) {
        for &next in rooms[node].iter() {
            let next = next as usize;
            if !vis[next] {
                vis[next] = true;
                Solution::step(next, vis, rooms);
            }
        }
    }
}