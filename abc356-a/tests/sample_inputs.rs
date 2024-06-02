use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("5 2 3")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1 3 2 4 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("7 1 1")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1 2 3 4 5 6 7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("10 1 10")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "10 9 8 7 6 5 4 3 2 1\n");
    assert!(output.stderr_str().is_empty());
}
