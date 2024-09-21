use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(".v.")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "v\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("chokudai")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "chokudai\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("...")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}
