//! LongMemEval harness. Dataset + paper: https://github.com/xiaowu0162/LongMemEval
//!
//! Plan: for each question, replay its chat history into a fresh
//! MemoryStore via `remember`, then `recall` with the question, hand the
//! hits to an LLM to answer, and grade the answer. Track accuracy per
//! question type and tokens of recalled context per question.

/// One evaluation item. Fill in fields from the dataset's JSON format.
#[derive(Debug, Clone)]
pub struct Item {
    pub question_id: String,
    pub question_type: String,
    pub question: String,
    pub answer: String,
    /// Sessions of (role, content) turns.
    pub sessions: Vec<Vec<(String, String)>>,
}

#[derive(Debug, Clone, Default)]
pub struct EvalReport {
    pub total: usize,
    pub correct: usize,
    pub mean_context_tokens: f64,
    // TODO: per-question-type breakdown.
}

pub fn load(path: &std::path::Path) -> std::io::Result<Vec<Item>> {
    let _ = path;
    todo!()
}

pub fn run(args: &[String]) {
    let _ = args;
    todo!()
}
