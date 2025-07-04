# ChromeOS QEMU Launcher 🚀


**`chromeos-launcher`** は、QEMUを使ったChromeOS仮想マシンの作成、実行、管理を簡単にするための、Rustで書かれたシンプルでパワフルなコマンドラインツールです。面倒なQEMUの引数設定からあなたを解放します。

-----

## ✨ 特徴

  * **直感的な操作**: `new`, `run`, `rm` といった分かりやすいサブコマンド。
  * **アーキテクチャ自動判別**: `x86_64` と `aarch64` のホストアーキテクチャを自動で検出し、適切なQEMUバイナリとUEFIファームウェアを使用します。
  * **柔軟なリソース設定**: CPUコア数やメモリを絶対値 (`4`, `8G`) またはホストに対する割合 (`50%`) で指定可能。
  * **設定ファイルの管理**: 各仮想マシンの設定を `~/.config/chromeos-launcher/` 以下に保存し、再利用を容易にします。
  * **UEFIファームウェアの自動検索**: `OVMF/AAVMF` ファイルをシステム標準のパスから自動で見つけ出します。
  * **高速・安全**: Rustによって書かれており、高速なパフォーマンスとメモリ安全性を実現しています。

-----

## 📦 インストール

### `cargo` を使用する場合 (推奨)

[Rustのツールチェイン](https://rustup.rs/)がインストールされていれば、`cargo` を使って簡単にインストールできます。

```bash
cargo install --git https://github.com/The-Infinitys/chromeos-launcher.git
```


### ソースからビルドする場合

1.  リポジトリをクローンします。

    ```bash
    git clone https://github.com/The-Infinitys/chromeos-launcher.git
    cd chromeos-launcher
    ```

2.  リリースモードでビルドします。

    ```bash
    cargo build --release
    ```

3.  完成したバイナリは `./target/release/chromeos-launcher` にあります。これをパスの通ったディレクトリ（例: `/usr/local/bin`）にコピーしてください。

    ```bash
    sudo cp ./target/release/chromeos-launcher /usr/local/bin/
    ```

-----

## 🚀 使い方

### ヘルプの表示

```bash
chromeos-launcher --help
# 各サブコマンドのヘルプ
chromeos-launcher new --help
```

### 1\. 新しい仮想マシンの作成 (`new`)

`new`コマンドは、新しい仮想マシンの設定ファイルを作成し、指定されたISOからインストールプロセスを開始します。

```bash
chromeos-launcher new \
  --name my-chrome-vm \
  --iso /path/to/chromeos.bin \
  --disk /home/user/vms/chrome.img \
  --disk-size 64G \
  --cpu-cores 50% \
  --memory 8G
```

  * `--disk` で指定したパスにファイルが存在しない場合、`--disk-size` で指定されたサイズのディスクイメージが自動的に作成されます。
  * `--disk` にはブロックデバイス (`/dev/sdb` など) も指定可能です。

### 2\. 仮想マシンの実行 (`run`)

作成済みの仮想マシンを起動します。

```bash
# 名前を指定して実行
chromeos-launcher run my-chrome-vm

# 引数を省略すると、最後に実行した仮想マシンが起動します
chromeos-launcher run
```

### 3\. 仮想マシンの削除 (`rm`)

仮想マシンの設定と、関連するディスクイメージを削除します。

```bash
chromeos-launcher rm my-chrome-vm
```

実行すると、まず設定ファイルの削除を確認するプロンプトが表示されます。ディスクがブロックデバイスではなく通常のファイルである場合、続けてディスクイメージを削除するかどうかの確認も行われます。

-----

## ⚙️ 設定ファイル

  * **設定ディレクトリ**: `~/.config/chromeos-launcher/`
  * **マシンごとの設定**: `~/.config/chromeos-launcher/machines/` 以下に、仮想マシンごとの設定ファイル（例: `my-chrome-vm.toml`）が保存されます。
  * **状態ファイル**: `~/.config/chromeos-launcher/last_run` には、最後に実行されたマシンの名前が記録されます。

-----

## 📜 ライセンス

このプロジェクトは、以下のいずれかのライセンスの下で提供されます。

  * **Apache License 2.0** ([LICENSE-APACHE](https://www.google.com/search?q=LICENSE-APACHE) または [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
  * **MIT License** ([LICENSE-MIT](https://www.google.com/search?q=LICENSE-MIT) または [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

あなたの選択に従います。

-----

## ❤️ 貢献

バグ報告や機能改善の提案は、GitHubのIssuesでいつでも歓迎しています！プルリクエストも大歓迎です。