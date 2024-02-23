use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

use crate::canvas::{self, request_animation_frame};

const NODE_SIZE: f64 = 20.0;
const VISITED: &str = "#FF0000";
const GRID_COLOR: &str = "#CCCCCC";
const PATH_COLOR: &str = "#7BD3EA";
const AVAILABLE_COLOR: &str = "#FFFFFF";
const WALL_COLOR: &str = "#000000";
const START_COLOR: &str = "#00FF00";
const END_COLOR: &str = "#0000FF";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);

}

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
    width: u32,
    height: u32,
    nodes: Vec<Node>,
    start_node_index: Option<usize>,
    end_node_index: Option<usize>,
    queue: VecDeque<usize>,
    is_backtracking: bool,
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

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_neighbor_indexes(&self, row: u32, column: u32) -> Vec<usize> {
        let mut indexes = Vec::with_capacity(8);
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
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

    pub fn bfs(&mut self) {
        if self.is_backtracking {
            self.backtracking();
            return;
        }
        if self.queue.is_empty() {
            return;
        }
        let current_node_index = self.queue.pop_front().unwrap();

        let (row, column) = (
            current_node_index as u32 / self.width,
            current_node_index as u32 % self.width,
        );

        if self.nodes[current_node_index].node_type() == Type::End {
            alert("End node found");

            self.queue.clear();

            self.queue.push_back(current_node_index);

            self.is_backtracking = true;
            return;
        }

        if self.nodes[current_node_index].node_type() == Type::Available {
            self.nodes[current_node_index].set_node_type(Type::Visited);
        }

        let neighbors = self.get_neighbor_indexes(row, column);

        for neighbor in neighbors {
            if self.nodes[neighbor].is_visited() {
                self.nodes[neighbor].weight = self.nodes[current_node_index].weight + 1;
                continue;
            }
            self.nodes[neighbor].set_visited();

            if self.nodes[neighbor].node_type() == Type::Wall {
                continue;
            }
            self.nodes[neighbor].weight = self.nodes[current_node_index].weight + 1;
            self.queue.push_back(neighbor);
        }
    }

    fn backtracking(&mut self) {
        if self.queue.is_empty() {
            return;
        }

        let current_node_index = self.queue.pop_front().unwrap();

        let (row, column) = (
            current_node_index as u32 / self.width,
            current_node_index as u32 % self.width,
        );

        let neighbors = self.get_neighbor_indexes(row, column);

        let mut visited_neighbors: Vec<usize> = vec![];

        for neighbor in neighbors {
            if self.nodes[neighbor].node_type() == Type::Start {
                alert("Start node found");
                self.queue.clear();
                return;
            }

            if self.nodes[neighbor].node_type() == Type::Visited {
                visited_neighbors.push(neighbor);
            }
        }

        let lowest_weight_index = visited_neighbors
            .iter()
            .min_by_key(|&&idx| self.nodes[idx].weight)
            .unwrap();

        self.nodes[*lowest_weight_index].set_node_type(Type::Path);
        self.queue.push_back(*lowest_weight_index);
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

                if nodes[index as usize].node_type() == Type::Available {
                    ctx.set_fill_style(&AVAILABLE_COLOR.into());
                } else if nodes[index as usize].node_type() == Type::Path {
                    ctx.set_fill_style(&PATH_COLOR.into());
                } else if nodes[index as usize].node_type() == Type::Start {
                    ctx.set_fill_style(&START_COLOR.into());
                } else if nodes[index as usize].node_type() == Type::End {
                    ctx.set_fill_style(&END_COLOR.into());
                } else if nodes[index as usize].node_type() == Type::Wall {
                    ctx.set_fill_style(&WALL_COLOR.into());
                } else if nodes[index as usize].node_type() == Type::Visited {
                    ctx.set_fill_style(&VISITED.into());
                }

                ctx.fill_rect(x, y, NODE_SIZE, NODE_SIZE);
            }
        }

        ctx.stroke();
    }
}

#[wasm_bindgen]
pub fn run_graph(document_id: &str, width: u32, height: u32) {
    let canvas = canvas::canvas(document_id);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut graph = Graph::new(width / NODE_SIZE as u32 - 1, height / NODE_SIZE as u32 - 1);

    graph.set_start_node(15, 20);
    graph.set_end_node(10, 20);
    graph.set_wall_node(13, 7);
    graph.set_wall_node(13, 8);
    graph.set_wall_node(13, 9);
    graph.set_wall_node(13, 10);
    graph.set_wall_node(13, 11);
    graph.set_wall_node(13, 12);
    graph.set_wall_node(13, 13);
    graph.set_wall_node(13, 14);
    graph.set_wall_node(13, 15);
    graph.set_wall_node(13, 16);
    graph.set_wall_node(13, 17);
    graph.set_wall_node(13, 18);
    graph.set_wall_node(13, 19);
    graph.set_wall_node(13, 20);
    graph.set_wall_node(13, 21);
    graph.set_wall_node(13, 22);
    graph.set_wall_node(13, 23);
    graph.set_wall_node(13, 24);
    graph.set_wall_node(13, 25);
    graph.set_wall_node(13, 26);
    graph.set_wall_node(13, 27);
    graph.set_wall_node(13, 28);
    graph.set_wall_node(13, 29);
    graph.set_wall_node(13, 30);
    graph.set_wall_node(13, 31);
    graph.set_wall_node(13, 32);
    graph.set_wall_node(13, 33);

    graph.draw_grid(&ctx);
    graph.draw_node(&ctx);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        graph.bfs();
        graph.draw_node(&ctx);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
