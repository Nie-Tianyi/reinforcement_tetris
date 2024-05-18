//! 游戏的逻辑都写在这个文件

use rand::{thread_rng, Rng, rngs::ThreadRng};

/// 玩家的动作, 有向左，向右，转向，向下，什么都不做（自由下落）
pub enum UserAction {
    Left, // 向左
    Right, // 向右
    Rotate, // 旋转
    Down, // 快速下落
    None, // 什么都不做，自由下落
}

/// 保存游戏状态的结构体，包括游戏的逻辑和所有数据
pub struct Tetris {
    main_view: [[bool;10];20], // 主游戏视图
    score: i64, // 分数
    next_shape: [[bool;4];4], // 下一个形状
    possible_shapes: Vec<[[bool;4];4]>, // 可能的形状
    rng: ThreadRng, // 随机数生成器，缓存起来防止每次使用时实例化一个新的
}

impl Tetris {
    /// 创建一个新的游戏实例
    pub fn new() -> Self {

        let possible_shapes = vec![
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false], // I
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false], // O
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false], // Z
            ],
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false], // S
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false], // L
            ],
        ];
        // 初始化随机数生成器
        let mut rng = thread_rng();
        // 取个0～4的随机数
        let next_shape_index: usize = rng.gen_range(0..5);
        // 决定第一个随机的形状
        let next_shape = possible_shapes[next_shape_index];

        Tetris {
            main_view: [[false;10];20],
            score: 0,
            next_shape,
            possible_shapes,
            rng,
        }
    }

    /// 获取游戏状态, 返回一个tuple（游戏主视图，玩家分数，下一个形状）
    pub fn view(&self) -> ([[bool;10];20], i64, [[bool;4];4]){
        (self.main_view, self.score, self.next_shape)
    }

    /// 传入一个动作，e.g. 向左，向右，转向，向下，什么都不做
    /// 根据游戏规则，更新游戏状态
    pub fn next(action: UserAction){
        match action {
            UserAction::Left => {
                // 向左移动
            }
            UserAction::Right => {
                // 向右移动
            }
            UserAction::Rotate => {
                // 旋转形状
            }
            UserAction::Down => {
                // 快速下落
            }
            UserAction::None => {
                // 自由下落
            }
        }
    }
}
