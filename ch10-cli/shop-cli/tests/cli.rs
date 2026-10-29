use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn workspace(csv: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("catalog.csv"), csv).unwrap();
    dir
}

fn shop(dir: &TempDir) -> Command {
    let mut command = Command::cargo_bin("shop-cli").unwrap();
    command.current_dir(dir.path());
    command
}

#[test]
fn price_of_two_mugs_exits_0() {
    let dir = workspace("id,title,price_cents\n1,mug,1000\n2,bowl,2500\n");
    let imported = shop(&dir).args(["import", "catalog.csv"]).output().unwrap();
    assert!(
        imported.status.success(),
        "import failed\n{}",
        String::from_utf8_lossy(&imported.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&imported.stdout), "read 2\n");
    let priced = shop(&dir)
        .args(["price", "1", "--qty", "2"])
        .output()
        .unwrap();
    assert!(
        priced.status.success(),
        "price failed\n{}",
        String::from_utf8_lossy(&priced.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&priced.stdout), "2000\n");
}

#[test]
fn bad_csv_row_names_the_line_on_stderr() {
    let dir = workspace("id,title,price_cents\n1,mug,abc\n");
    let imported = shop(&dir).args(["import", "catalog.csv"]).output().unwrap();
    let stderr = String::from_utf8_lossy(&imported.stderr);
    assert!(!imported.status.success(), "{stderr}");
    assert!(
        stderr.contains("catalog.csv") && stderr.contains("line: 2"),
        "{stderr}"
    );
}

#[test]
fn unknown_id_exits_non_zero() {
    let dir = workspace("id,title,price_cents\n1,mug,1000\n");
    let imported = shop(&dir).args(["import", "catalog.csv"]).output().unwrap();
    assert!(
        imported.status.success(),
        "import failed\n{}",
        String::from_utf8_lossy(&imported.stderr)
    );
    let priced = shop(&dir)
        .args(["price", "9", "--qty", "1"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&priced.stderr);
    assert!(!priced.status.success(), "{stderr}");
    assert!(stderr.contains("unknown id 9"), "{stderr}");
}
