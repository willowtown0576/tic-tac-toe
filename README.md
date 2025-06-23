# 🎮 Dioxus学習プロジェクト: 三目並べゲーム

Dioxusフレームワークを学ぶための三目並べ（tic-tac-toe）ゲームプロジェクトです。Rust + Dioxus + TailwindCSSを使用した現代的なWebアプリケーション開発の学習を目的としています。

## 🎯 学習目標

- **Dioxusコンポーネントシステム**の理解と実装
- **Rustでのリアクティブ状態管理**（signals）
- **TailwindCSSによるモダンスタイリング**
- **クロスプラットフォーム開発**（Web/Desktop/Mobile）
- **コンポーネント設計パターン**の習得

## 🛠️ 技術スタック

- **[Dioxus 0.6](https://dioxuslabs.com/)** - RustのReact風UIフレームワーク
- **[Rust](https://www.rust-lang.org/)** - システムプログラミング言語
- **[TailwindCSS v4](https://tailwindcss.com/)** - ユーティリティファーストCSSフレームワーク

## 🚀 クイックスタート

### 前提条件

- Rust (最新安定版)
- Node.js (TailwindCSS CLI用)
- [dx CLI](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli) (`cargo install dioxus-cli`)

### セットアップ

```bash
# プロジェクトクローン
git clone <repository-url>
cd tic-tac-toe

# Rust依存関係のインストール
cargo build

# TailwindCSS依存関係のインストール
npm install

# TailwindCSSのコンパイル
npx @tailwindcss/cli -i input.css -o assets/tailwind.css

# アプリケーション起動
dx serve
```

## 🎮 機能

- ✅ **直感的なゲームプレイ** - マス目をクリックして駒を配置
- ✅ **勝敗判定** - 縦・横・斜めの3つ揃いを自動検知
- ✅ **引き分け判定** - 全マス埋まり時の引き分け処理
- ✅ **ゲームリセット** - ワンクリックで新ゲーム開始
- ✅ **レスポンシブデザイン** - PC・タブレット・スマホ対応
- ✅ **モダンUI** - SVGアイコン、グラデーション、アニメーション
- ✅ **アクセシビリティ** - キーボード操作、代替テキスト対応

## 📁 プロジェクト構造

```
tic-tac-toe/
├── src/
│   ├── main.rs              # メインアプリケーション・ゲームロジック
│   └── components.rs        # 再利用可能UIコンポーネント
├── assets/
│   ├── tailwind.css        # コンパイル済みTailwindCSS
│   ├── favicon.ico         # ファビコン
│   ├── x-icon.svg         # Xプレイヤー用SVGアイコン
│   └── o-icon.svg         # Oプレイヤー用SVGアイコン
├── input.css              # TailwindCSSソースファイル
├── tailwind.config.js     # TailwindCSS設定
├── CLAUDE.md              # 開発者向け詳細ドキュメント
└── README.md              # このファイル
```

## 🧩 コンポーネント設計

### アーキテクチャ図

```
App (ルートコンポーネント)
└── TicTacToe (メインゲームコンポーネント)
    ├── GameStatus (ゲーム状態表示)
    ├── GameBoard (ゲーム盤面)
    │   └── GameCell × 9 (個別セル)
    └── ResetButton (リセットボタン)
```

### 主要コンポーネント

- **`App`** - ルートコンポーネント、グローバル設定
- **`TicTacToe`** - 状態管理、ゲームロジック、イベントハンドリング
- **`GameCell`** - 個別セル、プロパティ受け渡し、条件付きレンダリング
- **`GameBoard`** - レイアウト、CSSグリッド、ループレンダリング
- **`GameStatus`** - 動的コンテンツ表示、match式によるレンダリング分岐
- **`ResetButton`** - シンプルなイベントハンドリング

## 💡 学習ポイント

### Dioxus基礎
- `#[component]`属性によるコンポーネント定義
- `rsx!`マクロによる宣言的UI記述
- `use_signal`によるリアクティブ状態管理
- `EventHandler<T>`による型安全なイベント処理

### Rust活用
- `enum`による型安全な状態表現
- `match`式による包括的なパターンマッチング
- `Option<T>`による安全なnull表現
- クロージャによるイベントハンドリング

### TailwindCSS統合
- Rustファイル（.rs）のスキャン設定
- `@layer components`によるカスタムスタイル
- レスポンシブデザインの実装
- ホバー効果・アニメーションの追加

## 🔧 開発コマンド

```bash
# アプリケーション起動（デフォルト：デスクトップ）
dx serve

# プラットフォーム指定起動
dx serve --platform web      # Webブラウザ
dx serve --platform desktop  # デスクトップアプリ
dx serve --platform mobile   # モバイルアプリ

# TailwindCSS開発モード
npx @tailwindcss/cli -i input.css -o assets/tailwind.css --watch

# Rust関連
cargo build                  # ビルド
cargo clippy                 # 静的解析
cargo test                   # テスト実行
```

## 📚 学習リソース

- **[Dioxus公式ドキュメント](https://dioxuslabs.com/learn/0.6/)**
- **[Rust公式チュートリアル](https://doc.rust-lang.org/book/)**
- **[TailwindCSS公式ドキュメント](https://tailwindcss.com/docs)**
- **[CLAUDE.md](./CLAUDE.md)** - 詳細な開発者向けドキュメント

## 🎯 次のステップ（拡張アイデア）

1. **スコア機能** - 勝利回数の記録
2. **プレイヤー名設定** - カスタムプレイヤー名
3. **テーマ切り替え** - ダーク/ライトモード
4. **AI対戦** - コンピューター相手
5. **マルチプレイヤー** - オンライン対戦
6. **アニメーション強化** - 勝利演出、駒配置効果
7. **サウンド** - 効果音、BGM
8. **ゲーム履歴** - 過去のゲーム記録

---

**Happy Learning! 🦀✨**

