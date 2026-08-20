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
            r#"6 6 3
2 7 1 8 2 8
1 8 2 8 4 5
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "Yes\n");
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
            r#"1 1 1
43
1
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"1 1 1
100
100
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"12 15 12
748 169 586 329 972 529 432 519 408 587 138 249
656 114 632 299 984 755 404 772 155 506 832 854 353 465 387
"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}
