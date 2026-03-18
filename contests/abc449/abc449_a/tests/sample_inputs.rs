use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("2")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3.141592653589793\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"7"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "38.48451000647496\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"98"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "7542.9639612690935\n");
    assert!(output.stderr_str().is_empty());
}
