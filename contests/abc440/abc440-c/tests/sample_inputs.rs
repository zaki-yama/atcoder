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
8 2
1 10 10 1 1 10 10 1
8 3
1 10 10 1 1 10 10 1
8 4
1 10 10 1 1 10 10 1
4 100
100000 100000 100000 100000"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        "4
12
22
0\n"
    );
    assert!(output.stderr_str().is_empty());
}
