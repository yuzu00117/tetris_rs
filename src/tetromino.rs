use macroquad::prelude::Rect;
use macroquad::prelude::Color;
use crate::CELL_SIZE;

/// 7 種類のテトリミノ
#[derive(Clone, Copy)]
pub enum TetrominoType { I, O, T, S, Z, J, L }

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub kind: TetrominoType,
    pub rotation: usize,
    pub x: i32,
     pub y: i32,
 }

impl Tetromino {
    /// 4 回転分の形状データを返す
    pub fn shapes(kind: TetrominoType) -> [[(i32,i32); 4]; 4] {
        match kind {
            TetrominoType::I => [
                // 0° (水平)
                [(0,1),(1,1),(2,1),(3,1)],
                // 90° (垂直)
                [(2,0),(2,1),(2,2),(2,3)],
                // 180° (水平)
                [(0,2),(1,2),(2,2),(3,2)],
                // 270° (垂直)
                [(1,0),(1,1),(1,2),(1,3)],
            ],
            TetrominoType::O => [
                // O は回転しても同じ
                [(1,0),(2,0),(1,1),(2,1)],
                [(1,0),(2,0),(1,1),(2,1)],
                [(1,0),(2,0),(1,1),(2,1)],
                [(1,0),(2,0),(1,1),(2,1)],
            ],
            TetrominoType::T => [
                // 0° (頭上向き)
                [(1,0),(0,1),(1,1),(2,1)],
                // 90° (右向き)
                [(1,0),(1,1),(2,1),(1,2)],
                // 180° (下向き)
                [(0,1),(1,1),(2,1),(1,2)],
                // 270° (左向き)
                [(1,0),(0,1),(1,1),(1,2)],
            ],
            TetrominoType::S => [
                // 0°
                [(1,0),(2,0),(0,1),(1,1)],
                // 90°
                [(1,0),(1,1),(2,1),(2,2)],
                // 180°
                [(1,1),(2,1),(0,2),(1,2)],
                // 270°
                [(0,0),(0,1),(1,1),(1,2)],
            ],
            TetrominoType::Z => [
                // 0°
                [(0,0),(1,0),(1,1),(2,1)],
                // 90°
                [(2,0),(1,1),(2,1),(1,2)],
                // 180°
                [(0,1),(1,1),(1,2),(2,2)],
                // 270°
                [(1,0),(0,1),(1,1),(0,2)],
            ],
            TetrominoType::J => [
                // 0°
                [(0,0),(0,1),(1,1),(2,1)],
                // 90°
                [(1,0),(2,0),(1,1),(1,2)],
                // 180°
                [(0,1),(1,1),(2,1),(2,2)],
                // 270°
                [(1,0),(1,1),(0,2),(1,2)],
            ],
            TetrominoType::L => [
                // 0°
                [(2,0),(0,1),(1,1),(2,1)],
                // 90°
                [(1,0),(1,1),(1,2),(2,2)],
                // 180°
                [(0,1),(1,1),(2,1),(0,2)],
                // 270°
                [(0,0),(1,0),(1,1),(1,2)],
            ],
        }
    }

    /// 現在の回転・位置でのブロック矩形リストを返す
    pub fn blocks(&self) -> Vec<Rect> {
        let shape = Self::shapes(self.kind)[self.rotation % 4];
        shape.iter().map(|&(dx,dy)| {
            Rect::new(
                (self.x + dx) as f32 * CELL_SIZE,
                (self.y + dy) as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
            )
        }).collect()
    }

    pub fn new_random() -> Self {
        // ランダムなテトリミノを生成
        use macroquad::prelude::rand::gen_range;
        let kinds = [
            TetrominoType::I, TetrominoType::O, TetrominoType::T,
            TetrominoType::S, TetrominoType::Z, TetrominoType::J, 
            TetrominoType::L,
        ];

        // ランダムに選ぶ
        let kind = kinds[gen_range(0, kinds.len())];
        Tetromino {
            kind,
            rotation: 0,
            x: (crate::COLS as i32 / 2) - 2, // 中央に配置
            y: 0, // 上端からスタート
        }
    }
}

pub fn kind_color(kind: TetrominoType) -> Color {
    match kind {
        TetrominoType::I => Color::from_rgba(0, 255, 255, 255), // シアン
        TetrominoType::O => Color::from_rgba(255, 255, 0, 255), // 黄色
        TetrominoType::T => Color::from_rgba(128, 0, 128, 255), // 紫
        TetrominoType::S => Color::from_rgba(0, 255, 0, 255), // 緑
        TetrominoType::Z => Color::from_rgba(255, 0, 0, 255), // 赤
        TetrominoType::J => Color::from_rgba(0, 0, 255, 255), // 青
        TetrominoType::L => Color::from_rgba(255, 165, 0, 255), // オレンジ
    }
}
