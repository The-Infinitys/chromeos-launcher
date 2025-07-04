fn main() {
    // ターゲットOSがLinuxであるかを確認します。
    // Linux以外の場合、panic!を発生させてビルドを中断します。
    if !cfg!(target_os = "linux") {
        panic!("このアプリケーションはLinuxでのみ実行可能です。");
    }
    // ビルドスクリプトは、依存関係が変更されたときに再実行されるように、
    // 変更されたファイルをCargoに通知することがよくあります。
    // このシンプルなケースでは不要ですが、一般的な慣習として追加しておきます。
    println!("cargo:rerun-if-changed=build.rs");
}