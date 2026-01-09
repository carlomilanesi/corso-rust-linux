#[test]
fn test_with_one_argument() {
    let command = std::process::Command::new("target/debug/seq1")
        .arg("4")
        .output()
        .unwrap();
    assert_eq!(command.stdout, b"1\n2\n3\n4\n");
}

#[test]
fn test_with_two_arguments() {
    let command = std::process::Command::new("target/debug/seq1")
        .arg("6")
        .arg("9")
        .output()
        .unwrap();
    assert_eq!(command.stdout, b"6\n7\n8\n9\n");
}
