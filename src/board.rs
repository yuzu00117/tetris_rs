use macroquad::prelude::*;
use crate::{COLS, ROWS, CELL_SIZE};

/// セルに何色のブロックがあるか
pub struct Board {
    cells: [[Option<Color>; COLS]; ROWS],
}

impl Board {
    /// 空の盤面を作る
    pub fn new() -> Self {
        Self { cells: [[None; COLS]; ROWS] }
    }

    /// グリッド線を描画
    pub fn draw_grid(&self) {
        for y in 0..ROWS {
            for x in 0..COLS {
                let px = x as f32 * CELL_SIZE;
                let py = y as f32 * CELL_SIZE;
                draw_rectangle_lines(px, py, CELL_SIZE, CELL_SIZE, 1.0, DARKGRAY);
            }
        }
    }

    /// 固定済みセルを塗りつぶし
    pub fn draw_blocks(&self) {
        for y in 0..ROWS {
            for x in 0..COLS {
                if let Some(color) = self.cells[y][x] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        color,
                    );
                }
            }
        }
    }

    /// テトリミノを盤面に固定する
    pub fn lock_tetromino(&mut self, tet: &crate::tetromino::Tetromino, color: Color) {
        for block in tet.blocks() {
            let grid_x = (block.x / CELL_SIZE) as usize;
            let grid_y = (block.y / CELL_SIZE) as usize;
            if grid_y < ROWS && grid_x < COLS {
                self.cells[grid_y][grid_x] = Some(color);
            }
        }
    }

    /// テトリミノが盤面内かつ他のブロックと重なっていないか
    pub fn is_valid_position(&self, tet: &crate::tetromino::Tetromino) -> bool {
        for rect in tet.blocks() {
            let gx = (rect.x / CELL_SIZE) as i32;
            let gy = (rect.y / CELL_SIZE) as i32;
            // 範囲外はダメ
            if gx < 0 || gx >= COLS as i32 || gy < 0 || gy >= ROWS as i32 {
                return false;
            }
            // 他のブロックと重なっていたらダメ
            if self.cells[gy as usize][gx as usize].is_some() {
                return false;
            }
        }
        true
    }
    
    /// 埋まった行を消して、上の行を下に落とす。消した行数を返す
    pub fn clear_full_lines(&mut self) -> usize {
        let mut new_cells = [[None; COLS]; ROWS];
        let mut write_row = ROWS - 1;
        let mut cleared = 0;

        // 下から上へ一行ずつチェック
        for read_row in (0..ROWS).rev() {
            // この行が完全に埋まっているか
            if self.cells[read_row].iter().all(|c| c.is_some()) {
                cleared += 1;
                // スキップして write_row を変えない（行を消す）
            } else {
                // 埋まっていなければ write_row にコピー
                new_cells[write_row] = self.cells[read_row];
                write_row = write_row.saturating_sub(1);
            }
        }

        // 残り上部はすべて空行
        // (new_cells はすべて初期値がからなので不要)

        self.cells = new_cells;
        cleared
    }
}
