use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"4 5
2 0 3 0 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "2 1 3 4 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"3 7
1 1 0 0 0 0 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1 1 2 3 2 3 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"6 20
4 6 0 3 4 2 6 5 2 3 0 3 2 5 0 3 5 0 2 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        "4 6 1 3 4 2 6 5 2 3 1 3 2 5 1 3 5 4 2 6\n"
    );
    assert!(output.stderr_str().is_empty());
}
