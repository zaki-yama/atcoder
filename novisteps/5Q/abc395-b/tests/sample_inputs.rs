use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"11"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"###########
#.........#
#.#######.#
#.#.....#.#
#.#.###.#.#
#.#.#.#.#.#
#.#.###.#.#
#.#.....#.#
#.#######.#
#.........#
###########
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
        .output_with_stdin(r#"5"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"#####
#...#
#.#.#
#...#
#####
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"8"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"########
#......#
#.####.#
#.#..#.#
#.#..#.#
#.####.#
#......#
########
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(r#"2"#)
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(
        output.stdout_str(),
        r#"##
##
"#
    );
    assert!(output.stderr_str().is_empty());
}
