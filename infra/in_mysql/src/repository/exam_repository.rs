use pedu::domain::repository::exam_repository::ExamRepository;

pub struct ExamRepositoryImpl {}

impl ExamRepository for ExamRepositoryImpl {
    // fn new() -> Self {
    //     Self {}
    // }
    /// # 全ての試験を取得する
    fn all(&self) -> String {
        return String::from("{exams: [{name: exam1}, {name: exam1}]");
    }
}

/// # 全ての試験を取得する

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = all();
        // assert_eq!(result, "{exams: [{name: exam1}, {name: exam1}]");
    }
}
