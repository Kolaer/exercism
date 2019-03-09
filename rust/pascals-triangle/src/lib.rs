pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let row_count = self.row_count;

        let mut res = vec![];

        for row_id in 0..row_count {
            match row_id {
                0 => res.push(vec![1]),
                n => {
                    let prev_row = res.last().unwrap();
                    let mut this_row = Vec::with_capacity((n + 1) as usize);

                    this_row.push(1);
                    for window in prev_row.windows(2) {
                        this_row.push(window.iter().sum());
                    }

                    this_row.push(1);

                    res.push(this_row);
                }
            }
        }

        res
    }
}
