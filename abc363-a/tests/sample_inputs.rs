use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("123")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "77\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("250")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "50\n");
    assert!(output.stderr_str().is_empty());
}
