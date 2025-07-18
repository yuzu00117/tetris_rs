use macroquad::prelude::{
    BLACK, WHITE, clear_background, draw_rectangle, draw_text, get_frame_time, next_frame,
};
use std::collections::VecDeque;

// テトリスのゲームロジックを管理するモジュール
use crate::board::Board;
use crate::input::InputState;
use crate::tetromino::{Tetromino, TetrominoType};
use crate::{CELL_SIZE, COLS};

// 一回自動で1セル落下するまでの時間
const DROP_INTERVAL: f32 = 0.5; // 秒

const QUEUE_SIZE: usize = 5; // 次のピースキューのサイズ

// キーリピート定数
const DAS: f32 = 0.15; // 初動待機時間
const ARR: f32 = 0.12; // キーリピート間隔
pub struct Game {
    board: Board,                        // 盤面
    current: Tetromino,                  // 現在のテトリミノ
    input: InputState,                   // 入力状態
    drop_timer: f32,                     // 自動落下用タイマー
    score: usize,                        // スコア
    next_queue: VecDeque<TetrominoType>, // 次のピースキュー

    // 横移動用タイマー
    left_timer: f32,  // 左移動のタイマー
    right_timer: f32, // 右移動のタイマー
}

impl Game {
    pub fn new() -> Self {
        let mut queue = VecDeque::new();
        fill_bag(&mut queue);
        Self {
            board: Board::new(),              // 空の盤面を生成
            current: Tetromino::new_random(), // ランダムなテトリミノを生成
            input: InputState::poll(),        // 初期状態の入力
            drop_timer: 0.0,                  // 自動落下タイマー
            score: 0,                         // 初期スコアは0
            next_queue: queue,                // 次のピースキューを初期化
            left_timer: 0.0,                  // 左移動タイマー
            right_timer: 0.0,                 // 右移動タイマー
        }
    }

    pub async fn run(&mut self) {
        loop {
            // 1) 入力取得
            self.input = InputState::poll();

            // 2) ゲームロジック更新
            self.update();

            // 3) 描画
            clear_background(BLACK);
            self.board.draw_grid();
            self.board.draw_blocks();
            let c = crate::tetromino::kind_color(self.current.kind);
            for r in self.current.blocks() {
                draw_rectangle(r.x, r.y, r.w, r.h, c);
            }

            // ------------- Next プレビュー -------------
            let preview_x = COLS as f32 * CELL_SIZE + 20.0; // 盤面の右に余白
            let mut offset_y = 40.0;

            draw_text("Next:", preview_x, offset_y - 10.0, 24.0, WHITE);
            for &kind in &self.next_queue {
                // 仮の Tetromino で描画位置をずらして表示
                let preview = Tetromino {
                    kind,
                    rotation: 0,
                    x: 0,
                    y: 0,
                };
                let c = crate::tetromino::kind_color(kind);
                for rect in preview.blocks() {
                    // rect は (0..3) ブロックなので、
                    // 世界座標をプレビュー用にシフト
                    draw_rectangle(preview_x + rect.x, offset_y + rect.y, rect.w, rect.h, c);
                }
                offset_y += CELL_SIZE * 2.5; // 次のプレビューは下へ適度に間隔を
            }

            // スコア表示
            draw_text(
                &format!("Score: {}", self.score),
                10.0,  // X座標
                24.0,  // Y座標
                24.0,  // フォントサイズ
                WHITE, // 色
            );

            // 4) フレーム待機
            next_frame().await;
        }
    }

    fn update(&mut self) {
        let dt = get_frame_time(); // フレーム時間を取得

        // -------- LEFT ----------
        if self.input.left {
            // DAS フェーズ
            if self.left_timer == 0.0 {
                // 押した瞬間は即移動
                self.try_move(-1);
            }
            self.left_timer += dt;

            if self.left_timer >= DAS {
                // ARR フェーズ：インターバルカウンタ
                if self.left_timer - DAS >= ARR {
                    self.try_move(-1);
                    // ARR を超えたぶんを差し引き
                    self.left_timer = DAS; // ← ここを DAS に戻すのがコツ
                }
            }
        } else {
            self.left_timer = 0.0;
        }

        // -------- RIGHT ----------
        if self.input.right {
            if self.right_timer == 0.0 {
                self.try_move(1);
            }
            self.right_timer += dt;

            if self.right_timer >= DAS {
                if self.right_timer - DAS >= ARR {
                    self.try_move(1);
                    self.right_timer = DAS;
                }
            }
        } else {
            self.right_timer = 0.0;
        }

        // ------------その他の入力処理------------
        let mut candidate = self.current; // 試し置き用コピー

        // 左右移動
        if self.input.left {
            candidate.x -= 1;
            if self.board.is_valid_position(&candidate) {
                self.current.x -= 1; // 有効なら実際に移動
            }
        }
        if self.input.right {
            candidate.x += 1;
            if self.board.is_valid_position(&candidate) {
                self.current.x += 1;
            }
        }

        // 回転（上キーを一回押すたびに90度回転）
        if self.input.rotate {
            candidate.rotation = (candidate.rotation + 1) % 4;
            if self.board.is_valid_position(&candidate) {
                self.current.rotation = candidate.rotation;
            }
        }

        // ソフトドロップ（下キーを押している間は速く落下）
        if self.input.down {
            candidate.y += 1;
            if self.board.is_valid_position(&candidate) {
                self.current.y += 1;
            }
        }
        // 自動落下部分
        self.drop_timer += get_frame_time();
        if self.drop_timer >= DROP_INTERVAL {
            // ① 試し置き用コピー
            let mut candidate = self.current;
            candidate.y += 1;

            // ② 衝突しないなら current にだけ反映
            if self.board.is_valid_position(&candidate) {
                self.current.y += 1;
            } else {
                // ③ 衝突したら current はそのままの位置を固定
                //    → candidate は使わない！
                let col = crate::tetromino::kind_color(self.current.kind);
                self.board.lock_tetromino(&self.current, col);

                // ライン消去を呼んで、消した行数分だけスコアアップ
                let lines = self.board.clear_full_lines();
                if lines > 0 {
                    self.score += lines * 100; // 1行消すごとに100点
                }

                // 次ピース：キューから取り出す
                if let Some(next) = self.next_queue.pop_front() {
                    self.current = Tetromino {
                        kind: next,
                        rotation: 0,
                        x: (COLS as i32 / 2) - 2,
                        y: 0,
                    };
                }
                // キューに足りなくなったら 7種のピースを再度生成
                fill_bag(&mut self.next_queue);
            }
            self.drop_timer = 0.0;
        }
        // ------------------------------------------------------------
        // ハードドロップ（スペースキーで一気に落下）
        if self.input.hard_drop {
            let mut candidate = self.current;
            let mut drop_distance = 0;

            // 1マスずつ下へ試し置きし、おける限り下へ落とす
            while self.board.is_valid_position(&candidate) {
                drop_distance += 1;
                candidate.y += 1;
            }
            // 1マス上に戻す
            drop_distance -= 1;
            self.current.y += drop_distance;

            // 固定化
            let color = crate::tetromino::kind_color(self.current.kind);
            self.board.lock_tetromino(&self.current, color);

            // スコアにハードドロップボーナス
            self.score += drop_distance as usize * 2; // 1マスごとに2点

            // ライン消去
            let lines = self.board.clear_full_lines();
            if lines > 0 {
                self.score += lines * 100; // 1行消すごとに100
            }

            // 次ピース：キューから取り出す
            if let Some(next) = self.next_queue.pop_front() {
                self.current = Tetromino {
                    kind: next,
                    rotation: 0,
                    x: (COLS as i32 / 2) - 2, // 中央に配置
                    y: 0,
                };
            }

            // ハードドロップしたフレームでは自動落下・通常移動をスキップ
            return; // このフレームはここで終了
        }
    }

    fn try_move(&mut self, dx: i32) {
        let mut cand = self.current;
        cand.x += dx; // 仮の位置を計算
        if self.board.is_valid_position(&cand) {
            // 仮の位置が有効なら実際に移動
            self.current.x += dx;
        }
    }
}

// 七種類のミノをランダムシャッフルしてキューの後ろに補充
fn fill_bag(queue: &mut VecDeque<TetrominoType>) {
    use macroquad::prelude::rand::gen_range;
    let mut bag = vec![
        TetrominoType::I,
        TetrominoType::O,
        TetrominoType::T,
        TetrominoType::S,
        TetrominoType::Z,
        TetrominoType::J,
        TetrominoType::L,
    ];

    // Fisher-Yates シャッフル
    for i in (1..bag.len()).rev() {
        let j = gen_range(0, i + 1);
        bag.swap(i, j);
    }

    // QUEUE_SIZE までキューに補充
    while queue.len() < QUEUE_SIZE {
        if let Some(next) = bag.pop() {
            queue.push_back(next);
        }
        if bag.is_empty() {
            // bag が尽きたらまた補充
            bag = vec![
                TetrominoType::I,
                TetrominoType::O,
                TetrominoType::T,
                TetrominoType::S,
                TetrominoType::Z,
                TetrominoType::J,
                TetrominoType::L,
            ];
        }
    }
}
