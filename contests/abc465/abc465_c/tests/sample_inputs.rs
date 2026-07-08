use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"5
ooxoo
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "5 2 1 3 4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"7
ooooooo
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "7 5 3 1 2 4 6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"15
xooxoxoxoxoxxoo
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "15 11 10 7 6 3 1 2 4 5 8 9 12 13 14\n");
    assert!(output.stderr_str().is_empty());
}
