use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
#[ignore]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"4 3
3 1 4 5
2 1 3
1 1
2 2 3
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"8
9
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
            r#"5 7
1 2 4 8 16
2 1 5
1 4
1 5
2 1 5
2 2 4
1 1
2 3 3
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"31
31
7
4
"#
    );
    assert!(output.stderr_str().is_empty());
}
