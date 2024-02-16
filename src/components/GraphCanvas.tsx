import React, { useEffect } from 'react';
import init, { Graph } from 'wasm-libs';

const NODE_SIZE = 5;
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';

interface Props {
  width: number;
  height: number;
}

function drawGrid(ctx: CanvasRenderingContext2D) {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (NODE_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (NODE_SIZE + 1) + 1, (NODE_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (NODE_SIZE + 1) + 1);
    ctx.lineTo((NODE_SIZE + 1) * width + 1, j * (NODE_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const GraphCanvas: React.FunctionComponent<Props> = (props) => {
  useEffect(() => {
    let ignore = false;

    init().then(() => {
      if (ignore) {
        return;
      }

      const graph = new Graph();

      graph.set_start_node(0, 0);
      graph.set_end_node(10, 10);

      function renderLoop() {
        graph.bfs();
        requestAnimationFrame(renderLoop);
      }

      renderLoop();
    });

    return () => {
      ignore = true;
    };
  }, [props.height, props.width]);

  return <canvas id='graph-canvas' width={props.width} height={props.height} />;
};

export default GraphCanvas;
