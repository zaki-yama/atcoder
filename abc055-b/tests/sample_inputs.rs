use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("3")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("10")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3628800\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("100000")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "457992974\n");
    assert!(output.stderr_str().is_empty());
}
