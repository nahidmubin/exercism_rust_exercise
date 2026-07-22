#[derive(Debug)]
pub struct HighScores{
    scores_list: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores_list: scores.iter().copied().collect() }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores_list[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_list.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted_list = self.scores_list.clone();
        sorted_list.sort();
        sorted_list.pop()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_list = self.scores_list.clone();
        sorted_list.sort_by(|a, b| b.cmp(a));
        sorted_list.iter().take(3).cloned().collect::<Vec<u32>>()
    }
}
