import React, { useEffect } from 'react';
import init, { Boundary, run_random_circles_with_mouse_effets } from 'wasm-libs';

interface Props {
  width: number;
  height: number;
}

const CircleCanvas: React.FunctionComponent<Props> = (props) => {
  useEffect(() => {
    init().then(() => {
      run_random_circles_with_mouse_effets(
        'circle-canvas',
        new Boundary(props.width, props.height)
      );
    });
  }, [props.height, props.width]);

  return <canvas id='circle-canvas' width={props.width} height={props.height} />;
};

export default CircleCanvas;
