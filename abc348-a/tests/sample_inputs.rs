use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("7")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "ooxooxo\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("9")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "ooxooxoox\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#" "#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "\n");
    assert!(output.stderr_str().is_empty());
}
