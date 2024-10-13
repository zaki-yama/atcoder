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
            r#"2
1 2
-1 0"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "6.06449510224597979401\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"7
-14142 13562
-17320 50807
-22360 67977
24494 89742
-26457 51311
28284 27124
31622 77660"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "384694.57587932075868509383\n");
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
-100000 100000
100000 -100000
-100000 100000
100000 -100000
-100000 100000"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1414213.56237309504880168872\n");
    assert!(output.stderr_str().is_empty());
}
