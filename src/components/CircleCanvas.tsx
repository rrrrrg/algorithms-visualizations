import React, { useEffect } from 'react';
import init, { Boundary, run_random_circles_with_mouse_move_effets } from 'wasm-libs';

interface Props {
  width: number;
  height: number;
}

const CircleCanvas: React.FunctionComponent<Props> = (props) => {
  useEffect(() => {
    let ignore = false;

    init().then(() => {
      if (!ignore) {
        run_random_circles_with_mouse_move_effets(
          'circle-canvas',
          new Boundary(props.width, props.height),
          2000,
          10
        );
      }
    });

    return () => {
      ignore = true;
    };
  }, [props.height, props.width]);

  return <canvas id='circle-canvas' width={props.width} height={props.height} />;
};

export default CircleCanvas;
