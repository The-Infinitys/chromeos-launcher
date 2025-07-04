// src/main.rs

use std::collections::HashMap;
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

/// コマンドライン引数を解析し、HashMapとして返します。
///
/// この関数は、一般的なコマンドライン引数のパターンを認識します。
/// - `--flag`: 値が不要なフラグ。HashMapではキーのみが格納され、値は空文字列になります。
/// - `--key=value`: キーと値がイコールで結合された形式。
/// - `-k value`: ショートオプションとそれに続く値の形式。
///
/// # 戻り値
/// 解析された引数を表す `HashMap<String, String>`。
pub fn recognize_arg() -> HashMap<String, String> {
    let mut args_map = HashMap::new();
    // env::args() はプログラム名を含むすべての引数を返します。
    // 最初の引数 (プログラム名) は通常スキップします。
    let args: Vec<String> = env::args().collect();

    // 最初の引数 (プログラム名) はスキップし、インデックス1から解析を開始します。
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];

        if arg.starts_with("--") {
            // 例: --verbose, --output=file.txt
            if let Some(eq_pos) = arg.find('=') {
                // `--key=value` 形式の引数を解析します。
                let key = arg[2..eq_pos].to_string(); // `--` を除いたキー
                let value = arg[eq_pos + 1..].to_string(); // `=` の後の値
                args_map.insert(key, value);
            } else {
                // `--flag` 形式の引数 (値がない) を解析します。
                let key = arg[2..].to_string(); // `--` を除いたキー
                args_map.insert(key, "".to_string()); // フラグなので値は空文字列
            }
        } else if arg.starts_with("-") && arg.len() > 1 {
            // 例: `-v`, `-o file.txt` (ショートオプション)
            let key = arg[1..].to_string(); // `-` を除いたキー
            // 次の引数が存在し、それが別のオプション (`-` または `--` で始まる) でない場合、
            // それを現在のオプションの値と見なします。
            if i + 1 < args.len() && !args[i + 1].starts_with("-") {
                args_map.insert(key, args[i + 1].clone());
                i += 1; // 値を消費したので、次のループでインデックスをさらに1つ進める
            } else {
                // 値がない場合はフラグとして扱います。
                args_map.insert(key, "".to_string());
            }
        } else {
            // オプションではない引数 (例: コマンド名、ファイルパスなど) を処理します。
            // これらをどう処理するかはユースケースに依存しますが、
            // ここではシンプルに `_arg_N` の形式でキーを生成し、格納します。
            args_map.insert(format!("_arg_{}", i), arg.clone());
        }
        i += 1; // 次の引数へ進む
    }

    args_map
}
