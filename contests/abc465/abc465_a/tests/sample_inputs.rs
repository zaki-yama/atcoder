use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"316 465"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"101 248"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"666 999"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}
