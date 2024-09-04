use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_with_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().success().code(0).stdout("\n");

    Ok(())
}

#[test]
fn test_n_option() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(&["-n"]);
    cmd.assert().success().code(0).stdout("");

    Ok(())
}

#[test]
fn test_n_option_text() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(&["-n", "test  content", "\\t \\n \\v", "simple", "space"]);
    cmd.assert()
        .success()
        .code(0)
        .stdout("test  content \\t \\n \\v simple space");

    Ok(())
}

#[test]
fn test_e_option_text() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(&["-e", "test  content", "\\t \\n \\v", "simple", "space"]);
    cmd.assert()
        .success()
        .code(0)
        .stdout("test  content \t \n \x0b simple space\n");

    Ok(())
}
