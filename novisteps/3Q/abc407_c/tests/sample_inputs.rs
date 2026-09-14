use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"21"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"407"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "17\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"2025524202552420255242025524"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "150\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"202"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "15\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"2025"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "26\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"202552"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "28\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample7() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"312"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "16\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample8() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"0373"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "24\n");
    assert!(output.stderr_str().is_empty());
}
