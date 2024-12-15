#[derive(Clone)]
pub struct McqData {
    pub question: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub context: String,
}
