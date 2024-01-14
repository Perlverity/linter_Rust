# linter_Rust

## ScreenShot (Develop)
![image](https://github.com/Perlverity/linter_Rust/assets/68835326/465f5260-23f7-4856-bafd-cc69f7bf8659)

## Description
- Rustで作成するオリジナルのlinter
- Pythonのコーディング規約に即しているかチェックする
- N+1問題を解決する
- GitHubのコードレビューの負担を減らす

## Build
```bash
cargo build --release
```

## How to Use(binary file)
```bash
./rust_linter
```

## Task
- [x] Task 1: fetchメソッドがforループ内で処理しているかチェック
- [x] Task 2: 複数のpythonファイルでチェックできるようにする
- [ ] Task 3: if文の早期コンティニューができるかチェック
- [ ] Task 4: コーディング規約に即しているかチェック
- [ ] Task 5: GitHub Actionsで実行できるようにする
- [ ] Task 6: Rust単体テスト
- [ ] Task 7: 許容するAlertを管理する
