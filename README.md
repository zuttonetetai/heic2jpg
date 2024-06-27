# heic2jpg

HEICファイルをjpgファイルに変換するアプリ

## Installation

libheif >= 1.17がインストールされていないと動かない。※apt-getだと1.12.0がインストールされてしまう。

```bash
brew install libheif
```

その後

```bash
cargo build -r
mv ./target/release/heic2jpg ./
```

## Usage

変換したいHEICファイルをinput_heicフォルダに入れて上記で生成した実行ファイルを実行

```bash
./heic2jpg
```
