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

// コンポーネントモジュールをインポート
// 学習ポイント: モジュラー設計により再利用性と保守性を向上
mod components;
use components::{GameBoard, GameStatus, ResetButton};

// ============================================================================
// アセット定義（コンパイル時検証）
// ============================================================================
// asset!マクロを使用してコンパイル時にアセットの存在を検証
// これによりランタイムエラーを防ぎ、型安全性を確保
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const X_ICON: Asset = asset!("/assets/x-icon.svg");
const O_ICON: Asset = asset!("/assets/o-icon.svg");

// ============================================================================
// 型定義: プレイヤー
// ============================================================================
// Rustの列挙型（enum）を活用した型安全なプレイヤー表現
// 学習ポイント:
// - Clone, Copy: 値の複製を効率的に行う
// - PartialEq: 等価比較を可能にする
// - Debug: デバッグ出力を可能にする
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,  // プレイヤーX
    O,  // プレイヤーO
}

// Player enumのメソッド実装
// 学習ポイント: Rustのimpl文による型への機能追加
impl Player {
    /// プレイヤーの文字列表現を返す
    /// 学習ポイント: match式によるパターンマッチング
    pub fn symbol(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }

    /// プレイヤーのアイコンアセットを返す
    /// 学習ポイント: Asset型との統合、コンパイル時アセット検証
    pub fn icon(&self) -> Asset {
        match self {
            Player::X => X_ICON,
            Player::O => O_ICON,
        }
    }



    /// 次のプレイヤーを返す
    /// 学習ポイント: 状態遷移の実装、ゲームロジック
    pub fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

// ============================================================================
// 型定義: ゲーム状態
// ============================================================================
// ゲームの現在状態を表現する列挙型
// 学習ポイント: データを持つバリアント（Won(Player)）による表現力の向上
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,      // ゲーム中
    Won(Player),  // 勝利（どのプレイヤーが勝ったかを保持）
    Draw,         // 引き分け
}

// ============================================================================
// 型エイリアス: ゲーム盤面
// ============================================================================
// 3x3の2次元配列による盤面表現
// Option<Player>により空のセル（None）とプレイヤーが置かれたセル（Some(Player)）を区別
// 学習ポイント: Option型による安全なnull表現、多次元配列の活用
type Board = [[Option<Player>; 3]; 3];

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
    let mut board = use_signal(|| [[None; 3]; 3]);

    // 現在のプレイヤー（Xから開始）
    // 学習ポイント: enumを使った型安全な状態管理
    let mut current_player = use_signal(|| Player::X);

    // ゲームの現在状態（初期状態は「プレイ中」）
    // 学習ポイント: 複合的な状態を表現するenum
    let mut game_state = use_signal(|| GameState::Playing);

    // ============================================================================
    // ゲームロジック: 勝敗判定関数
    // ============================================================================
    // 純粋関数として実装された勝敗判定ロジック
    // 学習ポイント:
    // - 関数型プログラミングの原則（副作用なし）
    // - パターンマッチングの活用
    // - イテレータチェーンによる効率的な処理
    let check_winner = move |board: Board| -> GameState {
        // 横列をチェック（行ごとの勝敗判定）
        for i in 0..3 {
            if let (Some(a), Some(b), Some(c)) = (board[i][0], board[i][1], board[i][2]) {
                if a == b && b == c {
                    return GameState::Won(a);
                }
            }
        }

        // 縦列をチェック（列ごとの勝敗判定）
        for j in 0..3 {
            if let (Some(a), Some(b), Some(c)) = (board[0][j], board[1][j], board[2][j]) {
                if a == b && b == c {
                    return GameState::Won(a);
                }
            }
        }

        // 左上から右下への対角線をチェック
        if let (Some(a), Some(b), Some(c)) = (board[0][0], board[1][1], board[2][2]) {
            if a == b && b == c {
                return GameState::Won(a);
            }
        }

        // 右上から左下への対角線をチェック
        if let (Some(a), Some(b), Some(c)) = (board[0][2], board[1][1], board[2][0]) {
            if a == b && b == c {
                return GameState::Won(a);
            }
        }

        // 引き分け判定：全セルが埋まっているかチェック
        // 学習ポイント: イテレータチェーンとall()の活用
        if board.iter().flatten().all(|cell| cell.is_some()) {
            GameState::Draw
        } else {
            GameState::Playing
        }
    };

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
        let new_game_state = check_winner(board());
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
        board.set([[None; 3]; 3]);              // 盤面をクリア
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
