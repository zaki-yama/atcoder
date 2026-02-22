use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"5 5 3 4
#####
#...#
#.@.#
#..@#
#####
LLLDRUU"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "2 3 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"6 13 4 6
#############
#@@@@@@@@@@@#
#@@@@@@@@@@@#
#@@@@.@@@@@@#
#@@@@@@@@@@@#
#############
UURUURLRLUUDDURDURRR"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "3 11 11\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        // change here
        .output_with_stdin(
            r#"12 35 7 10
###################################
#.................................#
#..........@......................#
#......@................@.........#
#.............##............@.....#
#...##........##....##............#
#...##........##....##.......##...#
#....##......##......##....##.....#
#....##......##......##..##.......#
#.....#######.........###.........#
#.................................#
###################################
LRURRRUUDDULUDUUDLRLRDRRLULRRUDLDRU"#,
        )
        .tee_output()
        .expect_success();
    // change here
    assert_eq!(output.stdout_str(), "4 14 1\n");
    assert!(output.stderr_str().is_empty());
}
