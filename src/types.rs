// ============================================================================
// Dioxus学習プロジェクト: ゲーム型定義とロジック
// ============================================================================
// このファイルは三目並べゲームの型定義とゲームロジックを定義しています。
//
// 学習ポイント:
// - Rustモジュールシステムによるコード分離
// - 型定義とビジネスロジックの集約
// - enumとimpl文による型安全な設計
// - Option型とResult型の活用
// - 関数型プログラミングパターン

use dioxus::prelude::*;

// ============================================================================
// アセット定義（モジュール内で使用）
// ============================================================================
// asset!マクロを使用してコンパイル時にアセットの存在を検証
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
pub type Board = [[Option<Player>; 3]; 3];

// ============================================================================
// ゲームロジック: 勝敗判定システム
// ============================================================================
// 純粋関数として実装された勝敗判定ロジック
// 学習ポイント:
// - 関数型プログラミングの原則（副作用なし）
// - パターンマッチングの活用
// - イテレータチェーンによる効率的な処理
pub struct GameLogic;

impl GameLogic {
    /// ゲーム盤面から現在の状態を判定する
    /// 学習ポイント: 複合的な条件判定を段階的に実装
    pub fn check_game_state(board: Board) -> GameState {
        // 勝敗判定を実行
        if let Some(winner) = Self::check_winner(board) {
            return GameState::Won(winner);
        }

        // 引き分け判定：全セルが埋まっているかチェック
        if Self::is_board_full(board) {
            GameState::Draw
        } else {
            GameState::Playing
        }
    }

    /// 勝者がいるかチェックする
    /// 学習ポイント: Option型による安全な値の返却
    fn check_winner(board: Board) -> Option<Player> {
        // 横列をチェック（行ごとの勝敗判定）
        for row in 0..3 {
            if let Some(winner) = Self::check_line([
                board[row][0],
                board[row][1],
                board[row][2]
            ]) {
                return Some(winner);
            }
        }

        // 縦列をチェック（列ごとの勝敗判定）
        for col in 0..3 {
            if let Some(winner) = Self::check_line([
                board[0][col],
                board[1][col],
                board[2][col]
            ]) {
                return Some(winner);
            }
        }

        // 対角線をチェック（左上から右下）
        if let Some(winner) = Self::check_line([
            board[0][0],
            board[1][1],
            board[2][2]
        ]) {
            return Some(winner);
        }

        // 対角線をチェック（右上から左下）
        if let Some(winner) = Self::check_line([
            board[0][2],
            board[1][1],
            board[2][0]
        ]) {
            return Some(winner);
        }

        None
    }

    /// 3つのセルが同じプレイヤーで埋まっているかチェック
    /// 学習ポイント: 配列パターンマッチングと条件判定
    fn check_line(line: [Option<Player>; 3]) -> Option<Player> {
        match line {
            [Some(a), Some(b), Some(c)] if a == b && b == c => Some(a),
            _ => None,
        }
    }

    /// 盤面が満杯かどうかをチェック
    /// 学習ポイント: イテレータチェーンとall()の活用
    fn is_board_full(board: Board) -> bool {
        board.iter().flatten().all(|cell| cell.is_some())
    }

    /// 空の盤面を作成
    /// 学習ポイント: デフォルト値の提供
    pub fn empty_board() -> Board {
        [[None; 3]; 3]
    }

    /// 指定位置にプレイヤーの駒を配置可能かチェック
    /// 学習ポイント: バリデーション関数パターン
    pub fn is_valid_move(board: Board, row: usize, col: usize) -> bool {
        row < 3 && col < 3 && board[row][col].is_none()
    }

    /// 盤面に駒を配置する（新しい盤面を返す）
    /// 学習ポイント: 不変性を保つ関数型アプローチ
    pub fn make_move(mut board: Board, row: usize, col: usize, player: Player) -> Result<Board, &'static str> {
        if !Self::is_valid_move(board, row, col) {
            return Err("無効な手です");
        }

        board[row][col] = Some(player);
        Ok(board)
    }
}

// ============================================================================
// テスト: ゲームロジックの検証
// ============================================================================
// 学習ポイント: Rustのテスト駆動開発パターン
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = GameLogic::empty_board();
        assert_eq!(GameLogic::check_game_state(board), GameState::Playing);
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = GameLogic::empty_board();
        board[0][0] = Some(Player::X);
        board[0][1] = Some(Player::X);
        board[0][2] = Some(Player::X);

        assert_eq!(GameLogic::check_game_state(board), GameState::Won(Player::X));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = GameLogic::empty_board();
        board[0][0] = Some(Player::O);
        board[1][0] = Some(Player::O);
        board[2][0] = Some(Player::O);

        assert_eq!(GameLogic::check_game_state(board), GameState::Won(Player::O));
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = GameLogic::empty_board();
        board[0][0] = Some(Player::X);
        board[1][1] = Some(Player::X);
        board[2][2] = Some(Player::X);

        assert_eq!(GameLogic::check_game_state(board), GameState::Won(Player::X));
    }

    #[test]
    fn test_draw() {
        let board = [
            [Some(Player::X), Some(Player::O), Some(Player::X)],
            [Some(Player::O), Some(Player::O), Some(Player::X)],
            [Some(Player::O), Some(Player::X), Some(Player::O)],
        ];

        assert_eq!(GameLogic::check_game_state(board), GameState::Draw);
    }

    #[test]
    fn test_valid_move() {
        let board = GameLogic::empty_board();
        assert!(GameLogic::is_valid_move(board, 0, 0));
        assert!(!GameLogic::is_valid_move(board, 3, 0)); // 範囲外
    }

    #[test]
    fn test_make_move() {
        let board = GameLogic::empty_board();
        let result = GameLogic::make_move(board, 0, 0, Player::X);

        assert!(result.is_ok());
        let new_board = result.unwrap();
        assert_eq!(new_board[0][0], Some(Player::X));
    }

    #[test]
    fn test_player_next() {
        assert_eq!(Player::X.next(), Player::O);
        assert_eq!(Player::O.next(), Player::X);
    }
}
