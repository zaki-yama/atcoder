use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"20 25"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "2025\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"30 25"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3025\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"45 11"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3136\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"2025 1111"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "9834496\n");
    assert!(output.stderr_str().is_empty());
}
