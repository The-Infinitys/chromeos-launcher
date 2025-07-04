// src/main.rs

use std::env;
use std::path::PathBuf;
#[cfg(target_family = "unix")] // Linuxを含むUnix系OSに限定
use std::os::unix::fs::PermissionsExt; // 実行可能ビットのチェックに必要

/// 指定されたコマンドが環境変数PATH経由で利用可能かどうかをチェックします。
///
/// この関数はLinux (Unix系OS) での実行を想定しています。
/// 環境変数PATHの各ディレクトリを検索し、コマンド名に一致する実行可能ファイルが存在するかを確認します。
///
/// # 引数
/// * `cmd` - チェックするコマンド名 (`&str`)。
///
/// # 戻り値
/// コマンドが利用可能であれば `true`、そうでなければ `false`。
pub fn is_available(cmd: &str) -> bool {
    // 環境変数PATHを取得します。
    // PATHが設定されていない場合はNoneを返すので、その場合はfalseを返します。
    let path_env = match env::var_os("PATH") {
        Some(val) => val,
        None => return false,
    };

    // PATHをOS固有の区切り文字で分割し、各パスをイテレートします。
    for path_entry in env::split_paths(&path_env) {
        let mut full_path = PathBuf::from(path_entry);
        full_path.push(cmd); // コマンド名をパスに追加

        // ファイルが存在し、かつ実行可能であるかを確認します。
        // Unix系OSでは、ファイルが存在するだけでなく、実行可能ビットが立っている必要があります。
        if full_path.is_file() {
            // ファイルのメタデータを取得し、実行可能ビットをチェックします。
            // エラーが発生した場合は、そのパスは実行可能ではないと見なします。
            if let Ok(metadata) = full_path.metadata() {
                let permissions = metadata.permissions();
                // ユーザー、グループ、その他のいずれかで実行可能ビットが立っているかを確認
                if permissions.mode() & 0o111 != 0 {
                    return true;
                }
            }
        }
    }

    false // どのパスでも見つからなかった場合
}
