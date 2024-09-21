use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"3 2
2 1 3"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"1 1 2
2 1 1
2 1 3
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"1 2
1"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"5 5
2 3 2 3 2"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"1 1 1 1 1
1 2 2 3 2
1 3 1 3 2
1 3 2 2 2
1 3 2 3 1
2 1 2 3 2
2 2 1 3 2
2 2 2 2 2
2 2 2 3 1
2 3 1 2 2
2 3 1 3 1
2 3 2 1 2
2 3 2 2 1
"#
    );
    assert!(output.stderr_str().is_empty());
}
