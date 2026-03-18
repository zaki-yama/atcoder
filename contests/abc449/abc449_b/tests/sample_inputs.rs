use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"7 9 5
2 4
1 3
2 1
2 1
1 3
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"28
15
4
4
9
"#
    );
    assert!(output.stderr_str().is_empty());
}
