// src/error.rs

use colored::Colorize;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// アプリケーションで発生する可能性のあるエラーの種類を定義します。
// Debugトレイトを手動で実装するため、#[derive(Debug)]を削除
pub enum ErrorKind {
    /// IO関連のエラー (例: ファイルが見つからない、読み書きエラー)
    Io(io::Error),
    /// 数値解析エラー (例: 文字列を数値に変換できない)
    Parse(ParseIntError),
    /// コマンドが見つからない、または利用できないエラー
    CommandNotFound(String),
    /// 無効な引数エラー
    InvalidArguments(String),
    /// その他の汎用的なエラー
    Other(String),
}

/// アプリケーションのカスタムエラー型です。
/// エラーの種類と、必要に応じて詳細なメッセージを含みます。
// Debugトレイトを手動で実装するため、#[derive(Debug)]を削除
pub struct Error {
    kind: ErrorKind,
    message: Option<String>,
}

// ErrorKind に Debug を手動で実装します。
// Debug は通常、開発者向けの出力であり、詳細な情報を提供します。
impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io(err) => f.debug_tuple("Io").field(err).finish(),
            ErrorKind::Parse(err) => f.debug_tuple("Parse").field(err).finish(),
            ErrorKind::CommandNotFound(cmd) => f.debug_tuple("CommandNotFound").field(cmd).finish(),
            ErrorKind::InvalidArguments(arg) => {
                f.debug_tuple("InvalidArguments").field(arg).finish()
            }
            ErrorKind::Other(msg) => f.debug_tuple("Other").field(msg).finish(),
        }
    }
}

impl Error {
    /// 新しいErrorインスタンスを作成します。
    ///
    /// # 引数
    /// * `kind` - エラーの種類を示す `ErrorKind`。
    /// * `message` - エラーに関するオプションの詳細メッセージ。
    pub fn new(kind: ErrorKind, message: Option<String>) -> Self {
        Error { kind, message }
    }

    /// `ErrorKind::Other` を持つ新しいErrorインスタンスを簡単に作成するためのヘルパー関数です。
    ///
    /// # 引数
    /// * `message` - その他のエラーに関する詳細メッセージ。
    pub fn other(message: impl Into<String>) -> Self {
        Error {
            kind: ErrorKind::Other(message.into()),
            message: None,
        }
    }

    /// エラーメッセージを色付けされた文字列として返します。
    /// このメソッドは、`fmt::Display`の実装を利用して文字列をフォーマットします。
    ///
    /// # 戻り値
    /// 色付けされたエラーメッセージを含む `String`。
    pub fn to_colored_string(&self) -> String {
        format!("{}", self)
    }

    /// エラーの詳細を整形してフォーマッタに書き込みます。
    /// `fmt::Display`と`fmt::Debug`の両方から呼び出されます。
    fn display_for(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // エラーの種類に基づいて詳細を整形
        writeln!(f)?;
        let kind_str = match &self.kind {
            ErrorKind::Io(err) => {
                format!("IO Error - {}", err)
            }
            ErrorKind::Parse(err) => {
                format!("Parse Error - {}", err)
            }
            ErrorKind::CommandNotFound(cmd) => {
                format!("Command \"{}\" not found", cmd)
            }
            ErrorKind::InvalidArguments(arg_info) => {
                format!("Invalid Arguments - {}", arg_info)
            }
            ErrorKind::Other(desc) => desc.to_string(),
        };
        if !kind_str.is_empty() {
            writeln!(f, "  {}: {}", "Kind".cyan().bold(), kind_str)?;
        }
        if let Some(msg) = &self.message {
            let formatted_str = {
                let lines: Vec<String> = msg
                    .split("\n")
                    .map(|line| format!("    {}", line.trim()))
                    .collect();
                lines.join("\n")
            };
            writeln!(f, "  {}:|\n{}", "Message".green().bold(), formatted_str)?;
        }
        Ok(())
    }
}

/// `std::fmt::Debug` トレイトの手動実装。
/// `Error`構造体のデバッグ出力をカスタマイズします。
/// これは開発者向けの出力であり、`fmt::Display`とは異なる詳細な情報を提供します。
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // デバッグ出力もDisplayと同じ形式にする場合
        self.display_for(f)
    }
}

/// `std::fmt::Display` トレイトの実装。
/// これにより、`Error`型を `println!` や `format!` で整形して出力できるようになります。
/// エラーの種類に応じて色付けされた、整理された出力を提供します。
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // エラーのヘッダーを常に表示
        write!(f, "{}", "Error:".red().bold())?;
        self.display_for(f)
    }
}

/// `std::error::Error` トレイトの実装。
/// これにより、Rustのエラーエコシステムとの互換性が向上します。
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Io(err) => Some(err),
            ErrorKind::Parse(err) => Some(err),
            _ => None,
        }
    }
}

/// `From` トレイトの実装により、`std::io::Error` を `Error` に自動変換できるようにします。
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(ErrorKind::Io(err), None)
    }
}

/// `From` トレイトの実装により、`std::num::ParseIntError` を `Error` に自動変換できるようにします。
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::new(ErrorKind::Parse(err), None)
    }
}

/// `From` トレイトの実装により、`String` を `Error` に自動変換できるようにします。
/// これは、汎用的な「その他のエラー」を作成する際に便利です。
impl From<String> for Error {
    fn from(err_msg: String) -> Self {
        Error::new(ErrorKind::Other(err_msg), None)
    }
}

/// `From` トレイトの実装により、`&str` を `Error` に自動変換できるようにします。
/// これは、汎用的な「その他のエラー」を作成する際に便利です。
impl From<&str> for Error {
    fn from(err_msg: &str) -> Self {
        Error::new(ErrorKind::Other(err_msg.to_string()), None)
    }
}
