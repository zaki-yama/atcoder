use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"379"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "793 937\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"919"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "199 991\n");
    assert!(output.stderr_str().is_empty());
}
