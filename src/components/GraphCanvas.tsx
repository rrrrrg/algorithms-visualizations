import React, { useEffect } from 'react';
import init, { run_bfs } from 'wasm-libs';

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

      run_bfs('bfs-canvas', props.width, props.height);
    });

    return () => {
      ignore = true;
    };
  }, [props.height, props.width]);

  return <canvas id='bfs-canvas' width={props.width} height={props.height} />;
};

export default GraphCanvas;
