use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"700 300 9 17 7 21"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "7400\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"600 500 9 17 17 20"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1500\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"900 200 12 14 11 13"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1100\n");
    assert!(output.stderr_str().is_empty());
}
