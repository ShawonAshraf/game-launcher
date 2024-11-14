#[cfg(test)]
pub mod test_runner {
    use crate::runner::*;

    #[test]
    fn check_path_validation() {
        let invalid_path = String::from("exe.exe");
        let result = exe_runner::ExeRunner::check_for_valid_path(&invalid_path);

        assert_eq!(result, false);
    }
    
    #[test]
    fn test_run_invalid_path() {
        let invalid_path = String::from("exe.exe");
        let err = exe_runner::ExeRunner::new(invalid_path);
        
        // after the path validation it's safe to unwrap
        // because the struct won't give a new instance if the path is invalid.
        assert_eq!(err.err().unwrap(), "Invalid Path");
    }
}