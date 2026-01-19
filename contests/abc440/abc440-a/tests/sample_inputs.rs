use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"110 2"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "440\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"233 3"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1864\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"432"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "864\n");
    assert!(output.stderr_str().is_empty());
}
