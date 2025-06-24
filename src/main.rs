// ============================================================================
// Dioxus学習プロジェクト: 三目並べゲーム
// ============================================================================
// このファイルはDioxusフレームワークを学ぶための三目並べゲームのメインファイルです。
//
// 学習ポイント:
// - Dioxusコンポーネントの基本構造
// - use_signalによるリアクティブ状態管理
// - Rustの型システム（enum、pattern matching）を活用したゲームロジック
// - asset!マクロによる型安全なアセット管理
// - 関数型プログラミングパターンの活用

use dioxus::prelude::*;

// モジュール定義とインポート
// 学習ポイント: モジュラー設計により再利用性と保守性を向上
mod types;
mod components;

use types::{Player, GameState, GameLogic};
use components::{GameBoard, GameStatus, ResetButton};

// ============================================================================
// アセット定義（コンパイル時検証）
// ============================================================================
// asset!マクロを使用してコンパイル時にアセットの存在を検証
// これによりランタイムエラーを防ぎ、型安全性を確保
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



// ============================================================================
// メイン関数: アプリケーションエントリーポイント
// ============================================================================
fn main() {
    // Dioxusアプリケーションを起動
    // 学習ポイント: dioxus::launch関数によるアプリケーション初期化
    dioxus::launch(App);
}

// ============================================================================
// ルートコンポーネント: App
// ============================================================================
// アプリケーション全体のルートコンポーネント
// 学習ポイント:
// - #[component]属性によるDioxusコンポーネントの定義
// - document::Linkによるメタデータ設定
// - rsx!マクロによる宣言的UI記述
#[component]
fn App() -> Element {
    rsx! {
        // HTMLのheadセクションにファビコンを設定
        // 学習ポイント: ドキュメントレベルの設定とアセット参照
        document::Link { rel: "icon", href: FAVICON }

        // TailwindCSSスタイルシートを読み込み
        // 学習ポイント: 外部CSSの組み込み方法
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // メインコンテナ
        // 学習ポイント: TailwindCSSのみによる完全なスタイリング
        div {
            class: "w-screen h-screen flex items-center justify-center p-1 bg-gradient-to-br from-blue-700 to-indigo-800",

            // メインゲームコンポーネントを配置
            TicTacToe {}
        }
    }
}

// ============================================================================
// メインゲームコンポーネント: TicTacToe
// ============================================================================
// ゲームの状態管理とロジックを担当するコアコンポーネント
// 学習ポイント:
// - use_signalによるリアクティブ状態管理
// - クロージャによるイベントハンドリング
// - 純粋関数によるゲームロジック実装
#[component]
fn TicTacToe() -> Element {
    // ============================================================================
    // 状態管理: Dioxusシグナルによるリアクティブ状態
    // ============================================================================

    // ゲーム盤面の状態（3x3の2次元配列）
    // 学習ポイント: use_signalによる状態の初期化、自動再レンダリング
    let mut board = use_signal(|| GameLogic::empty_board());

    // 現在のプレイヤー（Xから開始）
    // 学習ポイント: enumを使った型安全な状態管理
    let mut current_player = use_signal(|| Player::X);

    // ゲームの現在状態（初期状態は「プレイ中」）
    // 学習ポイント: 複合的な状態を表現するenum
    let mut game_state = use_signal(|| GameState::Playing);

    // ============================================================================
    // イベントハンドラー: セルクリック処理
    // ============================================================================
    // セルがクリックされた時の処理
    // 学習ポイント:
    // - クロージャによるイベントハンドリング
    // - 状態の不変性を保つ更新パターン
    // - ゲームロジックとUIの分離
    let handle_cell_click = move |(row, col): (usize, usize)| {
        // 無効なクリックをガード（ゲーム終了時または既に置かれたセル）
        if game_state() != GameState::Playing || board()[row][col].is_some() {
            return;
        }

        // 盤面を更新（with_mutによる安全な変更）
        // 学習ポイント: with_mutによる状態の変更、借用チェッカーとの協調
        board.with_mut(|b| {
            b[row][col] = Some(current_player());
        });

        // 勝敗判定を実行
        let new_game_state = GameLogic::check_game_state(board());
        game_state.set(new_game_state);

        // ゲームが継続中なら次のプレイヤーに交代
        // 学習ポイント: 条件付き状態更新、プレイヤー交代ロジック
        if new_game_state == GameState::Playing {
            current_player.set(current_player().next());
        }
    };

    // ============================================================================
    // イベントハンドラー: ゲームリセット処理
    // ============================================================================
    // ゲームを初期状態にリセットする処理
    // 学習ポイント: 複数の状態を一括でリセットするパターン
    let reset_game = move |_| {
        board.set(GameLogic::empty_board());    // 盤面をクリア
        current_player.set(Player::X);          // プレイヤーをXにリセット
        game_state.set(GameState::Playing);     // ゲーム状態をプレイ中に
    };

    // ============================================================================
    // UI描画: rsx!マクロによる宣言的UI定義
    // ============================================================================
    // 学習ポイント:
    // - rsx!マクロの使い方
    // - コンポーネント間のプロパティ受け渡し
    // - TailwindCSSクラスとインラインスタイルの使い分け
    rsx! {
        // ゲームコンテナ（カードスタイル）
        div {
            class: "w-full max-w-sm mx-auto rounded-lg shadow-2xl p-3 bg-white/95 backdrop-blur-sm",

            // ゲームタイトル
            // 学習ポイント: TailwindCSSのグラデーションテキスト
            h1 {
                class: "text-xl font-bold text-center mb-3 bg-gradient-to-r from-blue-700 to-indigo-800 bg-clip-text text-transparent",
                "三目並べ"
            }

            // ゲーム状態表示コンポーネント
            // 学習ポイント: プロパティによるデータの受け渡し
            GameStatus {
                current_player: current_player(),
                game_state: game_state()
            }

            // ゲーム盤面コンポーネント
            // 学習ポイント: イベントハンドラーの受け渡し
            GameBoard {
                board: board(),
                game_state: game_state(),
                onclick: handle_cell_click
            }

            // リセットボタンコンポーネント
            // 学習ポイント: シンプルなイベントハンドリング
            ResetButton { onclick: reset_game }
        }
    }
}
