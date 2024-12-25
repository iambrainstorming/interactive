#[derive(Clone)]
pub struct McqData {
    pub question: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub context: String,
}

#[derive(Clone)]
pub struct McqDataMultipleSelect {
    pub question: String,
    pub correct_answers: Vec<String>,
    pub options: Vec<String>,
    pub context: String,
}
