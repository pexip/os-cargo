use cargo_test_support::compare::assert_ui;
use cargo_test_support::curr_dir;
use cargo_test_support::CargoCommand;
use cargo_test_support::Project;

use crate::cargo_remove::init_registry;

#[cargo_test]
#[ignore = "temporarily disabled for beta due to clap update"]
fn case() {
    init_registry();
    let project = Project::from_template(curr_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("remove")
        .args(["foo", "--flag"])
        .current_dir(cwd)
        .assert()
        .code(1)
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
