import React, { useEffect } from 'react';
import init, { start } from 'wasm-libs';

interface Props {}
const Canvas: React.FunctionComponent<Props> = (_props) => {
  useEffect(() => {
    init().then(() => {
      start();
    });
  }, []);
  return <canvas id="canvas" width={800} height={800} />;
};

export default Canvas;
