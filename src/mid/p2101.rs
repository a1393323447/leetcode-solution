struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let len = bombs.len();
        let mut explode = vec![vec![false; len]; len];

        for i in 0..len {
            for j in 0..len {
                let bombi = {
                    let bombi = &bombs[i];
                    (bombi[0] as i64, bombi[1] as i64, bombi[2] as i64)
                };
                let bombj = {
                    let bombj = &bombs[j];
                    (bombj[0] as i64, bombj[1] as i64)
                };
                explode[i][j] = Solution::interset(bombi, bombj);
            }
        }

        for i in 0..len {
            for j in 0..len {
                if explode[i][j] {
                    println!("{i} -> {j}");
                }
            }
        }

        let mut max_cnt = 0;
        let mut vis = vec![false; len];
        for i in 0..len {
            let cur_cnt = Solution::search(i, &mut vis, &explode);
            max_cnt = max_cnt.max(cur_cnt);
        }

        max_cnt
    }

    fn search(start: usize, vis: &mut [bool], explode: &Vec<Vec<bool>>) -> i32 {
        vis[start] = true;
        let mut cnt = 1;
        let mut stack = vec![start];
        while let Some(bomb_idx) = stack.pop() {
            let explode_bomb_idxs = explode[bomb_idx]
                .iter()
                .enumerate()
                .filter(|(_, &e)| e)
                .map(|(i, _)| i);
            for idx in explode_bomb_idxs {
                if !vis[idx] {
                    vis[idx] = true;
                    stack.push(idx);
                    cnt += 1;
                }
            }
        }

        vis.fill(false);

        cnt as i32
    }

    // 用于判断 bomb1 能否引爆 bomb2
    fn interset((x1, y1, r1): (i64, i64, i64), (x2, y2): (i64, i64)) -> bool {
        let diffx = x2 - x1;
        let diffy = y2 - y1;

        (diffx * diffx + diffy * diffy) <= (r1 * r1)
    }
}
