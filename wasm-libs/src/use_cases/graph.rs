use std::{cell::RefCell, fmt, rc::Rc};
use wasm_bindgen::prelude::*;

use crate::canvas::{self, request_animation_frame, Drawable};

const NODE_SIZE: f64 = 20.0;
const GRID_COLOR: &str = "#CCCCCC";
const PATH_COLOR: &str = "#FFFFFF";
const AVAILABLE_COLOR: &str = "#000000";
const WALL_COLOR: &str = "#FF0000";
const START_COLOR: &str = "#00FF00";
const END_COLOR: &str = "#0000FF";

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

        let nodes = (0..width * height).map(|_i| Node::Available).collect();

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

    pub fn draw_grid(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_stroke_style(&GRID_COLOR.into());

        // Vertical lines.
        for i in 0..self.width {
            ctx.move_to(i as f64 * (NODE_SIZE + 1.0) + 1.0, 0.0);
            ctx.line_to(
                i as f64 * (NODE_SIZE + 1.0) + 1.0,
                (NODE_SIZE + 1.0) * self.height as f64 + 1.0,
            );
        }

        // Horizontal lines.
        for j in 0..self.height {
            ctx.move_to(0.0, j as f64 * (NODE_SIZE + 1.0) + 1.0);
            ctx.line_to(
                (NODE_SIZE + 1.0) * self.width as f64 + 1.0,
                j as f64 * (NODE_SIZE + 1.0) + 1.0,
            );
        }

        ctx.stroke();
    }

    pub fn draw_node(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let nodes_ptr = self.nodes();

        let nodes =
            unsafe { std::slice::from_raw_parts(nodes_ptr, (self.width * self.height) as usize) };

        ctx.begin_path();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let x = column as f64 * (NODE_SIZE + 1.0) + 1.0;
                let y = row as f64 * (NODE_SIZE + 1.0) + 1.0;

                if nodes[index as usize] == Node::Available {
                    ctx.set_fill_style(&AVAILABLE_COLOR.into());
                } else if nodes[index as usize] == Node::Path {
                    ctx.set_fill_style(&PATH_COLOR.into());
                } else if nodes[index as usize] == Node::Start {
                    ctx.set_fill_style(&START_COLOR.into());
                } else if nodes[index as usize] == Node::End {
                    ctx.set_fill_style(&END_COLOR.into());
                } else if nodes[index as usize] == Node::Wall {
                    ctx.set_fill_style(&WALL_COLOR.into());
                }

                ctx.fill_rect(x, y, NODE_SIZE, NODE_SIZE);
            }
        }

        ctx.stroke();
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

impl Drawable for Graph {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        self.draw_grid(ctx);
        self.draw_node(ctx);
    }
}

#[wasm_bindgen]
pub fn run_graph(document_id: &str) {
    let canvas = canvas::canvas(document_id);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut graph = Graph::new();

    graph.set_start_node(0, 0);
    graph.set_end_node(20, 20);
    graph.set_wall_node(10, 10);
    graph.set_wall_node(10, 11);
    graph.set_wall_node(10, 12);
    graph.set_wall_node(10, 13);
    graph.set_wall_node(10, 14);
    graph.set_wall_node(10, 15);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        // graph.bfs();
        graph.draw(&ctx);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
