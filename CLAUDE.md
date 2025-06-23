# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a **Dioxus学習プロジェクト** - 三目並べ（tic-tac-toe）ゲームを通してDioxusフレームワークの使い方を学ぶためのプロジェクトです。Dioxus 0.6、Rust、TailwindCSS v4を使用した現代的なWeb/デスクトップアプリケーションです。

### 学習目標
- Dioxusコンポーネントシステムの理解
- Rustでのリアクティブ状態管理（signals）
- TailwindCSSによるモダンスタイリング
- クロスプラットフォーム開発
- コンポーネント設計パターン

## Architecture & Design Patterns
- **Component-based architecture**: コンポーネント志向設計で再利用性と保守性を重視
- **Separation of concerns**: ゲームロジックとUIコンポーネントの明確な分離
- **Asset management**: `asset!()`マクロによる型安全なアセット管理
- **Hybrid styling**: TailwindCSS + 必要に応じたインラインスタイル
- **Reactive state**: Dioxusシグナルによるリアクティブな状態更新
- **Cross-platform**: Web、デスクトップ、モバイル対応

## Development Commands

### Running the application
```bash
# Default platform (desktop)
dx serve

# Specific platforms
dx serve --platform web
dx serve --platform desktop
dx serve --platform mobile
```

### Tailwind CSS development
```bash
# Install Tailwind CSS CLI
npm install -D @tailwindcss/cli

# Compile Tailwind CSS (one-time)
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css

# Watch mode for development
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Cargo commands
```bash
# Build the project
cargo build

# Run clippy linter
cargo clippy

# Run tests
cargo test
```

## Project Structure & Key Files
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
├── tailwind.config.js     # TailwindCSS設定（コンテンツスキャン等）
├── package.json           # TailwindCSS CLI依存関係
├── Cargo.toml            # Rust依存関係・プラットフォーム設定
├── Dioxus.toml           # Dioxus固有設定
└── clippy.toml           # Dioxus向けClippy設定
```

### ファイル詳細
- **`src/main.rs`**: アプリケーションエントリーポイント、ゲーム状態管理、型定義
- **`src/components.rs`**: GameCell、GameBoard、GameStatus、ResetButtonコンポーネント
- **`input.css`**: TailwindCSSインポート + カスタムコンポーネントスタイル
- **`tailwind.config.js`**: Rustファイルをスキャンするよう設定
- **`clippy.toml`**: Dioxusシグナル用のawait最適化設定

## Dioxus Component Architecture (学習ポイント)

### コンポーネント階層
```
App (ルートコンポーネント)
└── TicTacToe (メインゲームコンポーネント)
    ├── GameStatus (ゲーム状態表示)
    ├── GameBoard (ゲーム盤面)
    │   └── GameCell × 9 (個別セル)
    └── ResetButton (リセットボタン)
```

### 各コンポーネントの役割と学習ポイント
- **`App`**: ルートコンポーネント、グローバル設定（CSS読み込み等）
- **`TicTacToe`**: 状態管理の中心、ゲームロジック、イベントハンドリング
- **`GameCell`**: プロパティ受け渡し、条件付きレンダリング、イベント処理
- **`GameBoard`**: レイアウトコンポーネント、CSSグリッド、ループレンダリング
- **`GameStatus`**: 動的コンテンツ表示、match式によるレンダリング分岐
- **`ResetButton`**: シンプルなイベントハンドリング

## Game Logic & State Management (学習ポイント)

### 型定義とパターンマッチング
```rust
// Rustの列挙型を活用した型安全な設計
enum Player { X, O }          // プレイヤー表現
enum GameState {              // ゲーム状態
    Playing,                  // プレイ中
    Won(Player),             // 勝利（どちらのプレイヤーが勝ったか）
    Draw                     // 引き分け
}
type Board = [[Option<Player>; 3]; 3];  // 3x3の盤面
```

### Dioxusシグナルによるリアクティブ状態管理
- **`use_signal`**: 状態の定義と自動再レンダリング
- **状態の読み取り**: `signal()`で現在の値にアクセス
- **状態の更新**: `signal.set(新しい値)`で状態を変更
- **状態の変更**: `signal.with_mut(|値| { /* 変更処理 */ })`で直接変更

## TailwindCSS + Dioxus統合 (学習ポイント)

### スタイリング戦略
1. **TailwindCSSクラス**: レイアウト、サイズ、基本的なスタイル
2. **インラインスタイル**: グラデーション、複雑な影効果
3. **コンポーネントクラス**: 再利用可能なスタイルパターン

### レスポンシブデザイン
- **ビューポート単位**: `vw`、`vh`で画面サイズに対応
- **アスペクト比**: セルの正方形を維持
- **フレキシブルレイアウト**: CSSグリッドで柔軟な配置

## Development Best Practices

### Dioxus開発のポイント
- **`rsx!`マクロ**: JSX風の宣言的UI記述
- **`EventHandler<T>`**: 型安全なイベントコールバック
- **`asset!()`マクロ**: コンパイル時アセット検証
- **プロパティ渡し**: 親から子への明示的なデータフロー

### 学習に推奨する実験
1. **新しいコンポーネント追加**: スコア表示、プレイヤー名入力など
2. **状態管理の拡張**: ゲーム履歴、設定の保存など
3. **スタイルのカスタマイズ**: テーマ切り替え、アニメーション追加
4. **インタラクションの改善**: キーボード操作、音効果など

## Common Development Tasks
- **新コンポーネント**: `src/components.rs`に追加→`src/main.rs`でインポート
- **ゲームロジック変更**: `TicTacToe`コンポーネント内の関数を修正
- **スタイル調整**: TailwindCSSクラスまたは`input.css`を編集
- **プラットフォーム固有ビルド**: `dx serve --platform <web|desktop|mobile>`