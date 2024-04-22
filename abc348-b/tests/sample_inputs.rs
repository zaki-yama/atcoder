use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"4
0 0
2 4
5 0
3 4
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"3
3
1
1
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
3 2
1 6
4 5
1 3
5 5
9 8
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"6
6
6
6
6
4
"#
    );
    assert!(output.stderr_str().is_empty());
}

// #[test]
// fn sample3() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(
//             r#"2
// 5 1 1
// 100 1 1
// "#,
//         )
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "No\n");
//     assert!(output.stderr_str().is_empty());
// }
