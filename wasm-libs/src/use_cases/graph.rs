use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Node {
    Start,
    End,
    Wall,
    Path,
    Available,
}

#[wasm_bindgen]
pub struct Graph {
    width: u32,
    height: u32,
    nodes: Vec<Node>,
    start_node_index: Option<usize>,
    end_node_indexd: Option<usize>,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Graph {
        let width = 64;
        let height = 64;

        let nodes = (0..width * height).map(|i| Node::Available).collect();

        Graph {
            width,
            height,
            nodes,
            start_node_index: None,
            end_node_indexd: None,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
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

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_available_neighbor_indexes(&self, row: u32, column: u32) -> Vec<usize> {
        let mut indexes = Vec::with_capacity(8);
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if self.nodes[idx] == Node::Available {
                    indexes.push(idx);
                }
            }
        }
        indexes
    }

    fn set_path_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.nodes[idx] = Node::Path;
    }

    pub fn set_start_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.start_node_index = Some(idx);
        self.nodes[idx] = Node::Start;
    }

    pub fn set_end_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.end_node_indexd = Some(idx);
        self.nodes[idx] = Node::End;
    }

    pub fn set_wall_node(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.nodes[idx] = Node::Wall;
    }

    pub fn bfs(&mut self) {
        let start_node_index = self.start_node_index.unwrap();
        let end_node_index = self.end_node_indexd.unwrap();

        let mut queue = Vec::new();
        queue.push(start_node_index);

        let mut came_from = vec![None; self.width as usize * self.height as usize];
        came_from[start_node_index] = Some(start_node_index);

        while !queue.is_empty() {
            let current = queue.remove(0);
            if current == end_node_index {
                break;
            }

            let current_row = current / self.width as usize;
            let current_col = current % self.width as usize;

            for neighbor in
                self.get_available_neighbor_indexes(current_row as u32, current_col as u32)
            {
                if came_from[neighbor].is_none() {
                    queue.push(neighbor);
                    came_from[neighbor] = Some(current);
                }
            }
        }

        let mut current = end_node_index;
        while current != start_node_index {
            let current_row = current / self.width as usize;
            let current_col = current % self.width as usize;
            self.set_path_node(current_row as u32, current_col as u32);
            current = came_from[current].unwrap();
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.nodes.as_slice().chunks(self.width as usize) {
            for &node in line {
                let symbol = match node {
                    Node::Start => "▶",
                    Node::End => "⏺",
                    Node::Wall => "✖️",
                    Node::Path => "◼",
                    Node::Available => "◻",
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
