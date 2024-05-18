//! 游戏的逻辑都写在这个文件

use std::fmt::{Debug, Formatter};
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
    score: i64, // 当前玩家分数
    next_shape: Shape, // 下一个形状
    possible_shapes: [Shape;7], // 可能的形状, 4个方块的坐标
    rng: ThreadRng, // 随机数生成器，缓存起来防止每次使用时实例化一个新的
}

#[derive(Debug,Copy, Clone)]
struct Shape {
    shape: [[bool;4];4], // 4个方块的形状
    position: [(usize,usize);4], // 4个方块在main view里的初始坐标
}

impl Tetris {
    /// 创建一个新的游戏实例
    pub fn new() -> Self {

        // 定义所有可能的形状
        let i_shape = Shape {
            shape: [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            position: [(3,0),(4,0),(5,0),(6,0)],
        };

        let l_shape = Shape {
            shape: [
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            position: [(3,0),(4,0),(5,0),(3,1)],
        };

        let j_shape = Shape {
            shape: [
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            position: [(3,0),(4,0),(5,0),(5,1)],
        };

        let o_shape = Shape {
            shape: [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            position: [(4,0),(5,0),(4,1),(5,1)],
        };

        let z_shape = Shape {
            shape: [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            position: [(3,0),(4,0),(4,1),(5,1)],
        };

        let s_shape = Shape {
            shape: [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            position: [(3,1),(4,1),(4,0),(5,0)],
        };

        let t_shape = Shape {
            shape: [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            position: [(3,1),(4,1),(5,1),(4,0)],
        };

        let possible_shapes = [i_shape, l_shape, j_shape, o_shape, z_shape, s_shape, t_shape];
        // 初始化主视图
        let mut main_view = [[false;10];20];
        // 初始化随机数生成器
        let mut rng = thread_rng();
        // 取个0～4的随机数
        let current_shape_index: usize = rng.gen_range(0..7);
        // 决定第一个随机的形状
        let current_shape = possible_shapes[current_shape_index];
        // 将第一个形状放到主视图中
        for (x,y) in current_shape.position{
            main_view[y][x] = true;
        }
        // 初始化下一个形状
        let next_shape_index: usize = rng.gen_range(0..7);
        let next_shape = possible_shapes[next_shape_index];
        println!("{:?}",possible_shapes);
        // 返回一个新的Tetris实例
        Tetris {
            main_view,
            score: 0,
            next_shape,
            possible_shapes,
            rng,
        }
    }

    /// 获取游戏状态, 返回一个tuple（游戏主视图，玩家分数，下一个形状）
    pub fn view(&self) -> ([[bool;10];20], i64, [[bool;4];4]){
        (self.main_view, self.score, self.next_shape.shape)
    }

    /// 判断游戏是否结束，如果结束，返回true
    /// 判断逻辑：
    /// 如果下一个方块初始位置跟现有的方块重合，那么游戏结束
    pub fn is_game_over(&self) -> bool {
        // 判断游戏是否结束
        false
    }

    /// 传入一个动作，e.g. 向左，向右，转向，向下，什么都不做
    /// 根据游戏规则，更新游戏状态
    pub fn next(&self,action: UserAction){
        match action {
            UserAction::Left => {
                // 向左移动
                self.move_left();
            }
            UserAction::Right => {
                // 向右移动
                self.move_right();
            }
            UserAction::Rotate => {
                // 旋转形状
                self.rotate();
            }
            UserAction::Down => {
                // 快速下落
                self.quick_down();
            }
            UserAction::None => {
                // 自由下落
                self.free_down();
            }
        }
    }

    fn move_left(&self) {
        // 向左移动
    }

    fn move_right(&self) {
        // 向右移动
    }

    fn rotate(&self) {
        // 旋转形状
    }

    fn quick_down(&self) {
        // 快速下落
    }

    fn free_down(&self) {
        // 自由下落
    }

    /// 检查方块左右是否越界，如果越界则返回true
    fn cross_boundary() -> bool {
        // 判断是否越界
        false
    }

    /// 检测是否碰撞其他方块，如果碰撞则返回true
    fn check_collision() -> bool {
        // 检查是否碰撞
        false
    }
}


/// 实现Debug trait，用于打印Debug时候的游戏状态
impl Debug for Tetris{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tetris:{{ \n")?;
        write!(f, "\tScore: {}\n", self.score)?;
        write!(f, "\tNext Shape: \n")?;
        for i in 0..4 {
            write!(f, "\t")?;
            for j in 0..4 {
                match self.next_shape.shape[i][j] {
                    true => {
                        write!(f, "  ■")?;
                    }
                    false => {
                        write!(f, "  □")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\tMain View:\n")?;
        for i in 0..20 {
            write!(f, "\t")?;
            for j in 0..10 {
                match self.main_view[i][j] {
                    true => {
                        write!(f, "  ■")?;
                    }
                    false => {
                        write!(f, "  □")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tetris = Tetris::new();
        print!("{:?}", tetris);
        assert_eq!(tetris.score, 0);
    }

    #[test]
    fn test_view() {
        let tetris = Tetris::new();
        let (main_view, score, next_shape) = tetris.view();
        assert_eq!(score, 0);
    }
}