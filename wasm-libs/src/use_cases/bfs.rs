use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

use crate::{
    canvas::{self, request_animation_frame},
    data_structures::graph::{Graph, Type},
};

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
trait Bfs {
    fn bfs(&mut self);

    fn backtracking(&mut self);
}

impl Bfs for Graph {
    fn bfs(&mut self) {
        if self.is_backtracking {
            self.backtracking();
            return;
        }
        if self.queue.is_empty() {
            return;
        }
        let current_node_index = self.queue.pop_front().unwrap();

        let (row, column) = (
            current_node_index as u32 / self.width(),
            current_node_index as u32 % self.width(),
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
            current_node_index as u32 / self.width(),
            current_node_index as u32 % self.width(),
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
}

trait GraphDrawable {
    fn draw_grid(&self, ctx: &web_sys::CanvasRenderingContext2d);
    fn draw_node(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

impl GraphDrawable for Graph {
    fn draw_grid(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_stroke_style(&GRID_COLOR.into());

        // Vertical lines.
        for i in 0..self.width() {
            ctx.move_to(i as f64 * (NODE_SIZE + 1.0) + 1.0, 0.0);
            ctx.line_to(
                i as f64 * (NODE_SIZE + 1.0) + 1.0,
                (NODE_SIZE + 1.0) * self.height() as f64 + 1.0,
            );
        }

        // Horizontal lines.
        for j in 0..self.height() {
            ctx.move_to(0.0, j as f64 * (NODE_SIZE + 1.0) + 1.0);
            ctx.line_to(
                (NODE_SIZE + 1.0) * self.width() as f64 + 1.0,
                j as f64 * (NODE_SIZE + 1.0) + 1.0,
            );
        }

        ctx.stroke();
    }

    fn draw_node(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let nodes_ptr = self.nodes();

        let nodes = unsafe {
            std::slice::from_raw_parts(nodes_ptr, (self.width() * self.height()) as usize)
        };

        ctx.begin_path();

        for row in 0..self.height() {
            for column in 0..self.width() {
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
pub fn run_bfs(document_id: &str, width: u32, height: u32) {
    let canvas = canvas::canvas(document_id);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut graph = Graph::new(width / NODE_SIZE as u32 - 1, height / NODE_SIZE as u32 - 1);

    graph.set_start_node(25, 20);
    graph.set_end_node(20, 30);
    graph.set_wall_node(23, 17);
    graph.set_wall_node(23, 18);
    graph.set_wall_node(23, 19);
    graph.set_wall_node(23, 20);
    graph.set_wall_node(23, 21);
    graph.set_wall_node(23, 22);
    graph.set_wall_node(23, 23);
    graph.set_wall_node(23, 23);
    graph.set_wall_node(24, 23);
    graph.set_wall_node(25, 23);
    graph.set_wall_node(26, 23);
    graph.set_wall_node(27, 23);
    graph.set_wall_node(28, 23);
    graph.set_wall_node(29, 23);
    graph.set_wall_node(23, 24);
    graph.set_wall_node(23, 25);
    graph.set_wall_node(23, 26);
    graph.set_wall_node(23, 28);
    graph.set_wall_node(23, 29);
    graph.set_wall_node(23, 30);
    graph.set_wall_node(23, 31);
    graph.set_wall_node(23, 32);
    graph.set_wall_node(23, 33);
    graph.set_wall_node(23, 34);
    graph.set_wall_node(23, 35);
    graph.set_wall_node(22, 35);
    graph.set_wall_node(21, 35);
    graph.set_wall_node(20, 35);
    graph.set_wall_node(19, 35);
    graph.set_wall_node(18, 35);
    graph.set_wall_node(17, 35);
    graph.set_wall_node(16, 35);
    graph.set_wall_node(25, 35);
    graph.set_wall_node(23, 36);
    graph.set_wall_node(23, 37);
    graph.set_wall_node(23, 38);
    graph.set_wall_node(23, 39);
    graph.set_wall_node(23, 40);
    graph.set_wall_node(23, 41);
    graph.set_wall_node(23, 42);
    graph.set_wall_node(23, 43);

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
