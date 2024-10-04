use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("1222")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "1+2+2+2=7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("0290")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "0-2+9+0=7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
#[ignore]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin("3242")
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3+2+4-2=7\n");
    assert!(output.stderr_str().is_empty());
}
