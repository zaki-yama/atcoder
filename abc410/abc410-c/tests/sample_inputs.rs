use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"5 5
2 3
1 2 1000000
3 4
2 2
2 3"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"3
1
1000000
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
            r#"1000000 2
1 1000000 999999
3 1000000000"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "");
    assert!(output.stderr_str().is_empty());
}
