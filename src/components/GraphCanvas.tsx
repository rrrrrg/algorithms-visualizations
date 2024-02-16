import React, { useEffect } from 'react';
import init, { Graph, Node } from 'wasm-libs';
import { memory } from 'wasm-libs/wasm_libs_bg.wasm';

const NODE_SIZE = 25;
const GRID_COLOR = '#CCCCCC';
const PATH_COLOR = '#FFFFFF';
const AVAILABLE_COLOR = '#000000';
const WALL_COLOR = '#FF0000';
const START_COLOR = '#00FF00';
const END_COLOR = '#0000FF';

interface Props {
  width: number;
  height: number;
}

const GraphCanvas: React.FunctionComponent<Props> = (props) => {
  useEffect(() => {
    let ignore = false;

    init().then(() => {
      if (ignore) {
        return;
      }

      const canvas = document.getElementById('graph-canvas') as HTMLCanvasElement;
      const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

      const graph = new Graph();

      graph.set_start_node(0, 0);
      graph.set_end_node(10, 10);

      function drawGrid() {
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        // Vertical lines.
        for (let i = 0; i <= graph.width(); i++) {
          ctx.moveTo(i * (NODE_SIZE + 1) + 1, 0);
          ctx.lineTo(i * (NODE_SIZE + 1) + 1, (NODE_SIZE + 1) * 64 + 1);
        }

        // Horizontal lines.
        for (let j = 0; j <= graph.height(); j++) {
          ctx.moveTo(0, j * (NODE_SIZE + 1) + 1);
          ctx.lineTo((NODE_SIZE + 1) * 64 + 1, j * (NODE_SIZE + 1) + 1);
        }

        ctx.stroke();
      }

      const getIndex = (row: number, column: number) => {
        return row * graph.width() + column;
      };

      const drawNode = () => {
        const nodesPtr = graph.nodes();
        const nodes = new Uint8Array(memory.buffer, nodesPtr, graph.width() * graph.height());

        ctx.beginPath();

        for (let row = 0; row < graph.height(); row++) {
          for (let column = 0; column < graph.width(); column++) {
            const index = getIndex(row, column);

            if (nodes[index] === Node.Available) {
              ctx.fillStyle = AVAILABLE_COLOR;
            } else if (nodes[index] === Node.Path) {
              ctx.fillStyle = PATH_COLOR;
            } else if (nodes[index] === Node.Start) {
              ctx.fillStyle = START_COLOR;
            } else if (nodes[index] === Node.End) {
              ctx.fillStyle = END_COLOR;
            } else if (nodes[index] === Node.Wall) {
              ctx.fillStyle = WALL_COLOR;
            }
          }
        }

        ctx.stroke();
      };

      function renderLoop() {
        graph.bfs();

        drawGrid();
        drawNode();

        requestAnimationFrame(renderLoop);
      }

      drawGrid();
      drawNode();
      requestAnimationFrame(renderLoop);
    });

    return () => {
      ignore = true;
    };
  }, [props.height, props.width]);

  return <canvas id="graph-canvas" width={props.width} height={props.height} />;
};

export default GraphCanvas;
