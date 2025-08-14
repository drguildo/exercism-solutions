#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(Vec::from(scores))
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().map(|score| score.clone())
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().map(|score| score.clone())
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.0.clone();
        sorted_scores.sort();
        sorted_scores
            .iter()
            .rev()
            .take(3)
            .map(|v| v.clone())
            .collect::<Vec<u32>>()
    }
}
