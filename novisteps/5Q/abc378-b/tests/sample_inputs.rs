use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"2
7 3
4 2
5
1 1
1 3
1 4
1 15
2 7"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"3
3
10
17
10
"#
    );
    assert!(output.stderr_str().is_empty());
}
