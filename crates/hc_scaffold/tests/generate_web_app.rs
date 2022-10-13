use assert_cmd::Command;

#[test]
fn scaffold_full_web_app_and_test_it() {
    let _r = std::fs::remove_dir_all("./tests/fixtures/forum");

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir("./tests/fixtures");
    let cmd = cmd.args(&["web-app", "forum"]);
    cmd.assert().success();

    let mut cmd = Command::new("nix-shell");
    let cmd = cmd.current_dir("./tests/fixtures/forum");
    let cmd = cmd.args(&["--run", "npm i && npm t"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir("./tests/fixtures/forum");
    let cmd = cmd.args(&["dna", "forum"]);
    cmd.assert().success();

    let mut cmd = Command::new("nix-shell");
    let cmd = cmd.current_dir("./tests/fixtures/forum");
    let cmd = cmd.args(&["--run", "npm t"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir("./tests/fixtures/forum");
    let cmd = cmd.args(&["zome", "posts"]);
    cmd.assert().success();

    let mut cmd = Command::new("nix-shell");
    let cmd = cmd.current_dir("./tests/fixtures/forum");
    let cmd = cmd.args(&["--run", "npm t"]);
    cmd.assert().success();
}
