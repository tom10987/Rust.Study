
### `cargo` によるビルド

* `Cargo.toml` という設定ファイルが必要。
  * `C` は必ず大文字でなければならない。
* ソースファイルは必ず `src` ディレクトリに置く必要がある。
  * `main()` 関数のあるファイル名は `main.rs` になっている必要がある。
    * 実行ファイルとしてビルドする場合のみ？
    * 設定次第でファイル名の変更はできるらしい。

---

* ビルド
```
$ cargo build
```

* 実行
```
$ ./target/debug/PROJECT_NAME
```

あるいは

```
$ cargo run
```

---

* `Cargo.toml`

```
[package]
name = "PROJECT_NAME"
version = "0.0.0"
author = "AUTHOR_NAME"
```

---

### `cargo` によるプロジェクトの作成

```
$ cargo new PROJECT_NAME --bin
```

* `--bin` で実行ファイルをビルドするプロジェクトとして作成できる。


