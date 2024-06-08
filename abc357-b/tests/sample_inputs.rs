use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"AtCoder"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "atcoder\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"SunTORY"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "SUNTORY\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"a"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "a\n");
    assert!(output.stderr_str().is_empty());
}
