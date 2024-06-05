use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("7 5 1")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("2 2 1")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("1 100 97")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("40 98 58")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "YES\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("77 42 36")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "NO\n");
    assert!(output.stderr_str().is_empty());
}
