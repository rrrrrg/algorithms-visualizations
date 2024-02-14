import React, { useEffect } from 'react';
import init, { draw_a_square } from 'wasm-libs';

interface Props {
  width: number;
  height: number;
}

const CircleCanvas: React.FunctionComponent<Props> = (props) => {
  useEffect(() => {
    let ignore = false;

    init().then(() => {
      if (!ignore) {
        draw_a_square();
      }
    });

    return () => {
      ignore = true;
    };
  }, [props.height, props.width]);

  return <canvas id="square-canvas" width={props.width} height={props.height} />;
};

export default CircleCanvas;
