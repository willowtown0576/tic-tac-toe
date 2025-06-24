// ============================================================================
// Dioxus学習プロジェクト: UIコンポーネント定義
// ============================================================================
// このファイルは三目並べゲームのUIコンポーネントを定義しています。
//
// 学習ポイント:
// - Dioxusコンポーネントの作成パターン
// - プロパティ（Props）の定義と活用
// - イベントハンドリング（EventHandler）
// - 条件付きレンダリング（match式の活用）
// - TailwindCSSによる完全なスタイリング（一本化）
// - レスポンシブデザインの実装

use dioxus::prelude::*;
use crate::{Player, GameState};

// ============================================================================
// GameCell コンポーネント: 個別ゲームセル
// ============================================================================
// 三目並べの個別セル（マス目）を表現するコンポーネント
//
// 学習ポイント:
// - Dioxusコンポーネントの基本構造
// - 複数のプロパティ受け取り
// - EventHandler<T>による型安全なイベント処理
// - 条件付きスタイリング
// - match式による条件付きレンダリング
#[component]
pub fn GameCell(
    // セルの行位置（0-2）
    row: usize,
    // セルの列位置（0-2）
    col: usize,
    // セルの値（None=空、Some(Player)=プレイヤーの駒）
    cell_value: Option<Player>,
    // 現在のゲーム状態
    game_state: GameState,
    // クリック時のイベントハンドラー（行、列のタプルを送信）
    onclick: EventHandler<(usize, usize)>
) -> Element {
    // セルが無効（クリック不可）かどうかを判定
    // 学習ポイント: 論理演算による状態の組み合わせ
    let is_disabled = game_state != GameState::Playing || cell_value.is_some();

    rsx! {
        // セルのボタン要素
        // 学習ポイント: 動的なクラス名生成とformat!マクロの活用
        button {
            class: format!(
                "aspect-square w-full min-w-16 min-h-16 border-2 rounded-lg flex items-center justify-center transition-all duration-200 {}",
                if is_disabled {
                    "cursor-not-allowed bg-gradient-to-br from-slate-50 to-slate-100 border-slate-300 shadow-inner"
                } else {
                    "cursor-pointer bg-gradient-to-br from-white to-slate-50 border-slate-400 shadow-md hover:-translate-y-1 hover:shadow-lg active:translate-y-0"
                }
            ),

            // クリックイベントハンドリング
            // 学習ポイント: ガード条件付きイベント処理、closure moveパターン
            onclick: move |_| if !is_disabled { onclick.call((row, col)) },

            // HTML属性の設定
            disabled: is_disabled,

            // セル内容の条件付きレンダリング
            // 学習ポイント: match式によるOption<T>の処理、動的コンテンツ
            match cell_value {
                // プレイヤーの駒がある場合：アイコン画像を表示
                Some(player) => rsx! {
                    img {
                        src: player.icon(),                           // Player enumのicon()メソッド呼び出し
                        class: "object-contain w-12 h-12",           // 画像フィット調整とサイズ指定（大きめに）
                        alt: format!("Player {}", player.symbol())   // アクセシビリティ対応
                    }
                },
                // 空のセルの場合：透明なスペーサー
                None => rsx! {
                    div {
                        class: "w-12 h-12"                          // レイアウト安定化のためのスペーサー
                    }
                }
            }
        }
    }
}

// ============================================================================
// GameBoard コンポーネント: ゲーム盤面
// ============================================================================
// 3x3のゲーム盤面全体を管理するレイアウトコンポーネント
//
// 学習ポイント:
// - CSSグリッドレイアウトの活用
// - ネストしたforループによるコンポーネント生成
// - レスポンシブデザイン（ビューポート単位の使用）
// - プロパティの透過的な受け渡し
// - コンテナコンポーネントパターン
#[component]
pub fn GameBoard(
    // ゲーム盤面の状態（3x3の2次元配列）
    board: [[Option<Player>; 3]; 3],
    // 現在のゲーム状態（セルの有効/無効判定に使用）
    game_state: GameState,
    // セルクリック時のイベントハンドラー（子コンポーネントに透過的に渡す）
    onclick: EventHandler<(usize, usize)>
) -> Element {
    rsx! {
        // ゲーム盤面のコンテナ
        // 学習ポイント: CSS Grid + TailwindCSSによるレスポンシブレイアウト
        div {
            class: "grid grid-cols-3 gap-2 mb-4 mx-auto aspect-square p-3 rounded-xl shadow-lg border-2 bg-gradient-to-br from-slate-100 to-slate-200 border-slate-400 w-80 max-w-[min(80vw,80vh)]",

            // ネストしたループによる9個のセル生成
            // 学習ポイント:
            // - Rustのrange記法（0..3）
            // - 2次元配列のインデックスアクセス
            // - コンポーネントの動的生成
            for row in 0..3 {
                for col in 0..3 {
                    GameCell {
                        row,                           // 行インデックス
                        col,                           // 列インデックス
                        cell_value: board[row][col],   // 該当セルの値
                        game_state,                    // ゲーム状態（透過的に渡す）
                        onclick                        // イベントハンドラー（透過的に渡す）
                    }
                }
            }
        }
    }
}

// ============================================================================
// GameStatus コンポーネント: ゲーム状態表示
// ============================================================================
// 現在のゲーム状態（プレイヤーターン、勝利、引き分け）を表示するコンポーネント
//
// 学習ポイント:
// - match式による複雑な条件分岐レンダリング
// - GameState enumの各バリアントに対応した表示
// - アニメーション効果の適用（TailwindCSS）
// - 動的なカラーリング（プレイヤーテーマカラー）
// - 情報表示コンポーネントパターン
#[component]
pub fn GameStatus(
    // 現在のプレイヤー（ターン表示に使用）
    current_player: Player,
    // ゲーム状態（表示内容の分岐に使用）
    game_state: GameState
) -> Element {
    rsx! {
        // ステータス表示のコンテナ
        // 学習ポイント: カード風スタイリング、中央揃えレイアウト
        div {
            class: "mb-3 p-2 rounded-lg flex items-center justify-center gap-2 border bg-gradient-to-br from-blue-50 to-indigo-50 border-indigo-200",

            // ゲーム状態に応じた表示内容の分岐
            // 学習ポイント:
            // - match式による包括的なパターンマッチング
            // - 各状態に特化したUI表現
            // - コンテクストに応じたアニメーション効果
            match game_state {
                // ゲーム進行中：現在のプレイヤーを表示
                GameState::Playing => rsx! {
                    img {
                        src: current_player.icon(),              // 現在プレイヤーのアイコン
                        class: "object-contain w-8 h-8",        // 画像フィット調整
                        alt: format!("Player {}", current_player.symbol())
                    }
                    span {
                        class: format!("text-lg font-semibold {}",
                            match current_player {
                                Player::X => "text-red-500",
                                Player::O => "text-blue-500",
                            }
                        ),
                        "現在のプレイヤー"
                    }
                },

                // 勝利状態：勝者を祝福表示
                GameState::Won(player) => rsx! {
                    img {
                        src: player.icon(),                      // 勝者のアイコン
                        class: "object-contain animate-bounce w-10 h-10", // バウンスアニメーションと大きめサイズ
                        alt: format!("Winner {}", player.symbol())
                    }
                    span {
                        class: format!("text-xl font-bold {}",   // 強調フォントと勝者テーマカラー
                            match player {
                                Player::X => "text-red-500",
                                Player::O => "text-blue-500",
                            }
                        ),
                        "勝利！"
                    }
                },

                // 引き分け状態：中立的な表示
                GameState::Draw => rsx! {
                    span {
                        class: "text-xl font-bold text-gray-500", // 強調フォントとグレー色（中立）
                        "🤝 引き分け!"                         // 絵文字で親しみやすく
                    }
                }
            }
        }
    }
}

// ============================================================================
// ResetButton コンポーネント: ゲームリセットボタン
// ============================================================================
// ゲームを初期状態にリセットするためのアクションボタン
//
// 学習ポイント:
// - シンプルなアクションコンポーネントパターン
// - EventHandler<()>による引数なしイベント処理
// - アクションボタンのUI/UXデザイン
// - アイコン + テキストの組み合わせレイアウト
// - ホバー効果（CSS）との連携
#[component]
pub fn ResetButton(
    // クリック時のイベントハンドラー（引数なし）
    onclick: EventHandler<()>
) -> Element {
    rsx! {
        // リセットボタン
        // 学習ポイント:
        // - フルワイズレイアウト（w-full）
        // - カスタムCSSクラス（reset-button）との連携
        // - TailwindCSSとインラインスタイルのハイブリッド
        button {
            class: "w-full text-white font-bold py-2 px-4 rounded-lg mt-4 flex items-center justify-center gap-2 text-base bg-gradient-to-br from-blue-700 to-indigo-800 shadow-blue-500/30 shadow-lg transition-all duration-200 hover:from-blue-800 hover:to-indigo-900 hover:shadow-blue-500/50 hover:shadow-xl border-none",

            // クリックイベントハンドリング
            // 学習ポイント: move closureによるイベント処理、引数の無視（_）
            onclick: move |_| onclick.call(()),

            // ボタン内容：アイコン + テキスト
            // 学習ポイント:
            // - 絵文字アイコンの活用
            // - flexレイアウトによる要素配置
            // - セマンティックな構造（span要素の使い分け）
            span { "🔄" }                    // リセットアイコン（絵文字）
            span { "新しいゲーム" }          // ボタンテキスト
        }
    }
}
