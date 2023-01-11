# poc-tauri

## 目的

tauriでアプリ開発を行うにあたり、バックエンド絡む処理の書き方について検証する

## 構成

Tauri + Vue 3 + TypeScript

## 検証機能

### 構造体によるemitスレッドの管理

バックエンドの処理の状態管理をどのように行うか検討

所有権問題で簡潔に書けないのいので保留

[rust - Rustで安全にthread loopを止める方法 - スタック・オーバーフロー](https://ja.stackoverflow.com/questions/53094/rust%e3%81%a7%e5%ae%89%e5%85%a8%e3%81%abthread-loop%e3%82%92%e6%ad%a2%e3%82%81%e3%82%8b%e6%96%b9%e6%b3%95)

### Stateにスレッドを持たせる

スレッド自体を構造体に持たせて、それをStateにいれtauri_commandからアクセスできるようにする

stateを操作するには&mutで借用する必要があるが、serde::Serializeトレイトを実装しないといけない

AppHandle<Wry>やArc<Mutex<32>>に対してderiveでは実装できないため保留

### tokioスレッドをtauriの外に建てる

[Tauri + Async Rust Process](https://rfdonnelly.github.io/posts/tauri-async-rust-process/)

## vue devtools

[Tauri+VueでVue Devtoolsを使う方法 - Qiita](https://qiita.com/mabasasi/items/0dae5d51e088f7e8d76d)

状態管理をvue側で行うなら欲しい。