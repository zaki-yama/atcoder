use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("1.012")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1.012\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("12.340")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "12.34\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("99.900")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "99.9\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("0.000")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}
