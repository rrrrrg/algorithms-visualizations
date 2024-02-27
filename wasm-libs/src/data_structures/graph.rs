use std::collections::VecDeque;

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Type {
    Start,
    End,
    Wall,
    Path,
    Visited,
    Available,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub weight: u32,
    node_type: Type,
    is_visited: bool,
}

impl Node {
    pub fn new() -> Node {
        Node {
            node_type: Type::Available,
            is_visited: false,
            weight: 0,
        }
    }

    pub fn node_type(&self) -> Type {
        self.node_type
    }

    pub fn is_visited(&self) -> bool {
        self.is_visited
    }

    pub fn set_visited(&mut self) {
        self.is_visited = true;
    }

    pub fn set_node_type(&mut self, node_type: Type) {
        self.node_type = node_type;
    }
}

pub struct Graph {
    pub queue: VecDeque<usize>,
    pub is_backtracking: bool,
    pub nodes: Vec<Node>,
    width: u32,
    height: u32,
    start_node_index: Option<usize>,
    end_node_index: Option<usize>,
}

impl Graph {
    pub fn new(width: u32, height: u32) -> Graph {
        let nodes = (0..width * height).map(|_| Node::new()).collect();

        Graph {
            width,
            height,
            nodes,
            start_node_index: None,
            end_node_index: None,
            queue: VecDeque::new(),
            is_backtracking: false,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn nodes(&self) -> *const Node {
        self.nodes.as_ptr()
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn get_neighbor_indexes(&self, row: u32, column: u32) -> Vec<usize> {
        let mut indexes = Vec::with_capacity(8);
        for delta_row in [0, self.height - 1, 1].iter().cloned() {
            for delta_col in [0, 1, self.width - 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                let idx = self.get_index(neighbor_row, neighbor_col);

                indexes.push(idx);
            }
        }
        indexes
    }

    pub fn set_start_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.start_node_index = Some(idx);
        self.nodes[idx].set_node_type(Type::Start);
        self.queue.push_back(idx)
    }

    pub fn set_end_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);

        self.end_node_index = Some(idx);
        self.nodes[idx].set_node_type(Type::End);
    }

    pub fn set_wall_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.nodes[idx].set_node_type(Type::Wall);
    }
}
