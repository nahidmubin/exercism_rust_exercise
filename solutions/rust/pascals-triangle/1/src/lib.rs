pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        for i in 0..row_count {
            
            let mut row: Vec<u32> = Vec::with_capacity((i+1) as usize);

            if i == 0 {
                row.push(1);
            }
            else if i == 1 {
                row.push(1);
                row.push(1);
            }
            else {
                let prev_row = &rows[i as usize -1];
                    let mut left_side_num = 0;
                    for j in prev_row {
                        row.push(left_side_num + j);
                        left_side_num = *j;
                    }
                    row.push(left_side_num);
            }
            

            rows.push(row);
        }

        Self {rows: rows}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
