//! 俄罗斯方块Tetris游戏的逻辑都写在这个文件

use std::fmt::{Debug, Formatter};
use std::ops::Div;
use rand::{thread_rng, Rng, rngs::ThreadRng};

/// 玩家的动作, 有向左，向右，转向，向下，什么都不做（自由下落）
pub enum UserAction {
    Left, // 向左
    Right, // 向右
    Rotate, // 旋转
    Down, // 快速下落
    None, // 什么都不做，自由下落
}

/// 保存每个可能的方块形状的结构体
#[derive(Debug,Copy, Clone)]
struct Shape {
    shape: [[u8;4];4], // 4个方块的形状，主要用来现实在next shape里
    position: [(usize,usize);4], // 4个方块在main view里的初始坐标

}


/// 保存游戏状态的结构体，包括游戏的逻辑和所有数据
/// main_view: 主游戏视图，20行10列，每个元素是一个u8
/// - 0 表示空
/// - 1 表示玩家操纵的方块
/// - 2 表示已经固定的方块
pub struct Tetris {
    main_view: [[u8;10];20], // 主游戏视图
    score: i64, // 当前玩家分数
    next_shape: Shape, // 下一个形状
    current_shape_coords: [(usize,usize);4], // 当前形状的坐标
    possible_shapes: [Shape;7], // 可能的形状, 4个方块的坐标
    rng: ThreadRng, // 随机数生成器，缓存起来防止每次使用时实例化一个新的
    is_game_over: bool, // 游戏是否结束
}

impl Tetris {
    /// 创建一个新的游戏实例
    pub fn new() -> Self {

        // 定义所有可能的形状
        let i_shape = Shape {
            shape: [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            position: [(3,0),(4,0),(5,0),(6,0)],
        };

        let l_shape = Shape {
            shape: [
                [0,0,0,0],
                [1,1,1,0],
                [1,0,0,0],
                [0,0,0,0],
            ],
            position: [(3,0),(4,0),(5,0),(3,1)],
        };

        let j_shape = Shape {
            shape: [
                [0,0,0,0],
                [1,1,1,0],
                [0,0,1,0],
                [0,0,0,0],
            ],
            position: [(3,0),(4,0),(5,0),(5,1)],
        };

        let o_shape = Shape {
            shape: [
                [0,0,0,0],
                [0,1,1,0],
                [0,1,1,0],
                [0,0,0,0],
            ],
            position: [(4,0),(5,0),(4,1),(5,1)],
        };

        let z_shape = Shape {
            shape: [
                [0,0,0,0],
                [1,1,0,0],
                [0,1,1,0],
                [0,0,0,0],
            ],
            position: [(3,0),(4,0),(4,1),(5,1)],
        };

        let s_shape = Shape {
            shape: [
                [0,0,0,0],
                [0,1,1,0],
                [1,1,0,0],
                [0,0,0,0],
            ],
            position: [(5,0),(4,0),(3,1),(4,1)],
        };

        let t_shape = Shape {
            shape: [
                [0,1,0,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            position: [(3,1),(4,1),(5,1),(4,0)],
        };

        let possible_shapes = [i_shape, l_shape, j_shape, o_shape, z_shape, s_shape, t_shape];
        // 初始化主视图
        let mut main_view = [[0_u8;10];20];
        // 初始化随机数生成器
        let mut rng = thread_rng();
        // 取个0～4的随机数
        let current_shape_index: usize = rng.gen_range(0..7);
        // 决定第一个随机的形状
        let current_shape = possible_shapes[current_shape_index];
        // 将第一个形状的坐标更新到current_shape_coords
        let current_shape_coords = current_shape.position;
        // 将第一个形状放到主视图中
        for (x,y) in current_shape.position{
            main_view[y][x] = 1_u8;
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
            current_shape_coords,
            possible_shapes,
            rng,
            is_game_over: false,
        }
    }

    /// 获取游戏状态, 返回一个tuple（游戏主视图，玩家分数，下一个形状）
    pub fn view(&self) -> ([[bool;10];20], i64, [[bool;4];4]){
        let main_view_binary_graph = self.main_view.map(|row| {
            row.map(|x| x != 0)
        });

        let next_shape_binary_graph = self.next_shape.shape.map(|row| {
            row.map(|x| x != 0)
        });

        (main_view_binary_graph, self.score, next_shape_binary_graph)
    }

    /// 判断游戏是否结束，如果结束，返回true
    /// 判断逻辑：
    /// 如果下一个方块初始位置跟现有的方块重合，那么游戏结束
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    /// 传入一个动作，e.g. 向左，向右，转向，向下，什么都不做
    /// 根据游戏规则，更新游戏状态
    pub fn next(&mut self,action: UserAction){
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

    fn move_left(&mut self) {
        // 向左移动
        for (x,y) in self.current_shape_coords {
            if x == 0 || self.main_view[y][x-1] == 2 {
                return;
            }
        }

    }

    fn move_right(&mut self) {
        // 向右移动
        for (x,y) in self.current_shape_coords {
            if x == 9 || self.main_view[y][x+1] == 2 {
                return;
            }
        }
    }

    fn rotate(&mut self) {
        // 旋转形状，顺时针旋转90度
        let new_coords = Self::spin_90_degree(self.current_shape_coords);

        for (x,y) in new_coords {
            // 检查旋转后的形状是否超出边界或者碰撞了其他方块
            if x > 9 || y > 19 || self.main_view[y][x] == 2 {
                // 如果超出边界或者碰撞了其他方块，不做任何操作
                return;
            }
        }
        // 清除原先的形状
        for (x,y) in self.current_shape_coords {
            self.main_view[y][x] = 0;
        }
        // 更新新的形状
        for (x,y) in new_coords {
            self.main_view[y][x] = 1;
        }
        self.current_shape_coords = new_coords;
    }

    fn spin_90_degree(coords: [(usize, usize); 4]) -> [(usize,usize);4] {

        //如果是正方形，则什么都不动
        if coords[0].0 == coords[1].0 // 第一个点的x等于第二个点的x
            && coords[0].1 == coords[2].1 // 第一个点的y等于第三个点的y
            && coords[0].0 + 1 == coords[3].0 // 第一个点的x比第四个点的x大一
            && coords[0].1 + 1 == coords[3].1 // 第一个点的y比第四个点的y大一
        {
            return coords;
        }

        // 将图像围绕中心（第二个坐标）顺时针旋转90度
        // 先把所有坐标转换为小数f64
        let coords_float: Vec<(f64, f64)> = coords.iter().map(|&(x, y)| (x as f64, y as f64)).collect();
        // 以第二个坐标为旋转中心
        let (cx, cy) = coords_float[1];
        // 计算所有点相对于旋转中心的坐标
        let coords_relative: Vec<(f64, f64)> = coords_float.iter().map(|&(x, y)| {
            let x = x - cx;
            let y = y - cy;
            (x, y)
        }).collect();
        // 计算旋转90度后相对坐标
        let new_coords_relative: Vec<(f64, f64)> = coords_relative.iter().map(|&(x, y)| {
            (y, -x)
        }).collect();
        // 计算旋转90度后的绝对坐标，四舍五入，转换为usize
        let new_coords: [(usize, usize); 4] = new_coords_relative.iter().map(|&(x, y)| {
            let x = (x + cx).round() as usize;
            let y = (y + cy).round() as usize;
            (x, y)
        }).collect::<Vec<(usize, usize)>>().try_into().unwrap();
        new_coords
    }

    fn quick_down(&mut self) {
        // 快速下落
         loop {
            for (x,y) in self.current_shape_coords{
                if y + 1 > 19{
                    return;
                }
                if self.main_view[y+1][x] == 2  {
                    return;
                }
            }
            self.free_down();
        }
    }

    fn free_down(&mut self) {
        // 自由下落
        // 检查下落后是否碰撞了其他方块 或者 达到底部
        for (x,y) in self.current_shape_coords {
            if y + 1 > 19 || self.main_view[y+1][x] == 2 {
                // 如果碰撞了，将当前方块固定到主视图中
                for (x,y) in self.current_shape_coords {
                    self.main_view[y][x] = 2;
                }
                // 检查是否有满行, 返回消除的行数
                let full_rows = self.check_full_rows();
                // 更新分数
                self.update_score(full_rows);
                // 更新下一个形状
                self.update_next_shape();
                return;
            }
        }
        // 如果没有碰撞，更新当前形状的坐标
        // 先将当前形状的坐标清空
        for (x,y) in self.current_shape_coords {
            self.main_view[y][x] = 0;
        }
        // 将当前形状的坐标向下移动一格
        for (x,y) in self.current_shape_coords {
            // 更新主视图
            self.main_view[y+1][x] = 1;
        }
        // 更新current_shape_coords
        self.current_shape_coords = self.current_shape_coords.map(|(x,y)| (x,y+1));
    }

    fn check_full_rows(&mut self) -> i64 {
        let mut res = 0;
        // 检查是否有满行
        for i in 0..20 {
            if self.main_view[i].iter().all(|&x| x == 2) {
                // 如果有满行，将满行的方块清空
                for j in 0..10 {
                    self.main_view[i][j] = 0;
                }
                // 满行数加1
                res += 1;
                // 将满行上面的固定方块向下移动
                for k in (0..i).rev() {
                    for j in 0..10 {
                        if self.main_view[k][j] != 1 && self.main_view[k+1][j] != 1{
                            self.main_view[k+1][j] = self.main_view[k][j];
                        }
                    }
                }
            }
        }
        res
    }

    fn update_score(&mut self, full_rows: i64) {

        // 更新分数
        // 如果没有消除，则不变
        if full_rows == 0 {
            return;
        }
        // 每消除一行，得10分，如果一次消除多行，得分更高
        // 同时消除一行得10分，消除两行得30分，消除三行得80分，消除四行得150分
        self.score += 10 * (full_rows * full_rows - 1);
    }

    fn update_next_shape(&mut self) {
        // 更新下一个形状
        // 判断下一个方块的初始位置是否有方块，如果有，则游戏结束
        for (x,y) in self.next_shape.position {
            if self.main_view[y][x] == 2 {
                self.is_game_over = true;
                return;
            }
        }
        // 如果没有碰撞，将下一个形状放到主视图中
        for (x,y) in self.next_shape.position {
            self.main_view[y][x] = 1;
        }
        // 更新当前形状的坐标
        self.current_shape_coords = self.next_shape.position;
        // 更新下一个形状
        let next_shape_index: usize = self.rng.gen_range(0..7);
        self.next_shape = self.possible_shapes[next_shape_index];
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
                    0 => {
                        write!(f, "  □")?;
                    }
                    _ => {
                        write!(f, "  ■")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\tMain View:\n")?;
        for i in 0..20 {
            write!(f, "\t")?;
            for j in 0..10 {
                write!(f," {}", self.main_view[i][j])?;
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
    fn test_rotate(){
        let mut tetris = Tetris::new();
        print!("{:?}", tetris);
        tetris.next(UserAction::None);
        print!("{:?}", tetris);
        tetris.next(UserAction::None);
        print!("{:?}", tetris);
        tetris.next(UserAction::None);
        print!("{:?}", tetris);
        tetris.rotate();
        print!("{:?}", tetris);
    }

    #[test]
    fn test_quick_down() {
        let mut tetris = Tetris::new();
        print!("{:?}", tetris);
        tetris.quick_down();
        print!("{:?}", tetris);
        tetris.next(UserAction::None);
        print!("{:?}", tetris);
    }

    #[test]
    fn test_check_full_row() {
        let mut tetris = Tetris::new();

        tetris.main_view[17] = [2,2,2,2,0,2,0,2,2,2];
        tetris.main_view[18] = [2,2,2,2,2,2,2,2,2,2];
        tetris.main_view[19] = [2,2,2,2,2,2,2,2,2,2];
        let full_rows = tetris.check_full_rows();
        assert_eq!(full_rows, 2);
        print!("{:?}", tetris);
    }
}