use std::collections::BTreeMap;
use PipeType::*;

pub type Pipes = BTreeMap<Node, PipeType>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Grid {
    pub pipes: Pipes,
    pub start_pipe: Node,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn build(lines: &[String]) -> Grid {
        let mut pipes = BTreeMap::new();
        let mut start_pipe = None;

        let rows = lines.len();
        let cols = lines[0].len();

        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let node = Node {
                    row: row as isize,
                    col: col as isize,
                };

                let pipe_type = match char {
                    '|' => Vertical,
                    '-' => Horizontal,
                    'L' => UpRightBend,
                    'J' => UpLeftBend,
                    '7' => DownLeftBend,
                    'F' => DownRightBend,
                    'S' => {
                        start_pipe = Some(node);
                        continue;
                    }
                    _ => continue,
                };

                pipes.insert(node, pipe_type);
            }
        }

        // Figure out the Start pipe's type
        let start_pipe = start_pipe.unwrap();
        pipes.insert(start_pipe, Grid::start_pipe_type(&pipes, &start_pipe));

        Grid {
            pipes,
            start_pipe,
            rows,
            cols,
        }
    }

    pub fn start_pipe_type(pipes: &Pipes, start_pipe: &Node) -> PipeType {
        let mut connected = vec![];

        if let Some(up) = pipes.get(&start_pipe.up()) {
            if up.goes_down() {
                connected.push(start_pipe.up());
            }
        }

        if let Some(down) = pipes.get(&start_pipe.down()) {
            if down.goes_up() {
                connected.push(start_pipe.down());
            }
        }

        if let Some(left) = pipes.get(&start_pipe.left()) {
            if left.goes_right() {
                connected.push(start_pipe.left());
            }
        }

        if let Some(right) = pipes.get(&start_pipe.right()) {
            if right.goes_left() {
                connected.push(start_pipe.right());
            }
        }

        if connected == Grid::connected_nodes(start_pipe, Vertical) {
            Vertical
        } else if connected == Grid::connected_nodes(start_pipe, Horizontal) {
            Horizontal
        } else if connected == Grid::connected_nodes(start_pipe, UpRightBend) {
            UpRightBend
        } else if connected == Grid::connected_nodes(start_pipe, UpLeftBend) {
            UpLeftBend
        } else if connected == Grid::connected_nodes(start_pipe, DownLeftBend) {
            DownLeftBend
        } else if connected == Grid::connected_nodes(start_pipe, DownRightBend) {
            DownRightBend
        } else {
            panic!();
        }
    }

    pub fn connected_nodes(node: &Node, pipe_type: PipeType) -> [Node; 2] {
        match pipe_type {
            Vertical => [node.up(), node.down()],
            Horizontal => [node.left(), node.right()],
            UpRightBend => [node.up(), node.right()],
            UpLeftBend => [node.up(), node.left()],
            DownLeftBend => [node.down(), node.left()],
            DownRightBend => [node.down(), node.right()],
        }
    }

    pub fn connected_pipes(&self, node: &Node) -> [Node; 2] {
        Grid::connected_nodes(node, self.pipes[node])
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
pub struct Node {
    pub row: isize,
    pub col: isize,
}

impl Node {
    pub fn up(&self) -> Node {
        Node {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn down(&self) -> Node {
        Node {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(&self) -> Node {
        Node {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(&self) -> Node {
        Node {
            row: self.row,
            col: self.col + 1,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PipeType {
    Vertical,
    Horizontal,
    UpRightBend,
    UpLeftBend,
    DownLeftBend,
    DownRightBend,
}

impl PipeType {
    pub fn goes_up(&self) -> bool {
        matches!(self, Vertical | UpRightBend | UpLeftBend)
    }

    pub fn goes_down(&self) -> bool {
        matches!(self, Vertical | DownLeftBend | DownRightBend)
    }

    pub fn goes_left(&self) -> bool {
        matches!(self, Horizontal | UpLeftBend | DownLeftBend)
    }

    pub fn goes_right(&self) -> bool {
        matches!(self, Horizontal | UpRightBend | DownRightBend)
    }
}
