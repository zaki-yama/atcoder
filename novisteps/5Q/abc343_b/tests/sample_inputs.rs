use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"4
0 1 1 0
1 0 0 1
1 0 0 0
0 1 0 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"2 3
1 4
1
2
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
            r#"2
0 0
0 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"

"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"5
0 1 0 1 1
1 0 0 1 0
0 0 0 0 1
1 1 0 0 1
1 0 1 1 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"2 4 5
1 4
5
1 2 5
1 3 4
"#
    );
    assert!(output.stderr_str().is_empty());
}
