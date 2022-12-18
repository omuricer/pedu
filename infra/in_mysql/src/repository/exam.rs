/// # 全ての試験を取得する
pub fn all() -> &'static str {
    return "{exams: [{name: exam1}, {name: exam1}]";
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = all();
        assert_eq!(result, "{exams: [{name: exam1}, {name: exam1}]");
    }
}
