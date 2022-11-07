use assert_cmd::Command;
use temp_dir::TempDir;

#[test]
fn scaffold_full_web_app_and_test_it() {
    let tempdir = TempDir::new().unwrap();

    let tempdir_path = tempdir.path().to_path_buf();

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir(&tempdir_path);
    let cmd = cmd.args(&[
        "web-app",
        "forum",
        "--setup-nix",
        "true",
        "--template",
        "lit",
    ]);
    cmd.assert().success();

    let apptempdir = tempdir_path.join("forum");

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir(&apptempdir);
    let cmd = cmd.args(&["dna", "forum"]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir(&apptempdir);
    let cmd = cmd.args(&[
        "zome",
        "posts",
        "--integrity",
        "dnas/forum/zomes/integrity",
        "--coordinator",
        "dnas/forum/zomes/coordinator",
    ]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("hc-scaffold").unwrap();
    let cmd = cmd.current_dir(&apptempdir);
    let cmd = cmd.args(&["entry-type", "post", "posts", "--crud", "crud", "--fields"]);
    cmd.assert().success();

    let mut cmd = Command::new("nix-shell");
    let cmd = cmd.current_dir(&apptempdir);
    let cmd = cmd.args(&["--run", "npm i && npm t"]);
    cmd.assert().success();
}
