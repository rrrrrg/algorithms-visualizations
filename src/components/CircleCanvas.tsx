import React, { useEffect, useRef } from 'react';
import init, { Boundary, Circle, Coordinate, Velocity } from 'wasm-libs';

interface Props {
  width: number;
  height: number;
}

const CircleCanvas: React.FunctionComponent<Props> = (props) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  function getRandomColor(): string {
    const COLORS = ['#F9EFDB', '#EBD9B4', '#9DBC98', '#638889'];

    return COLORS[Math.floor(Math.random() * COLORS.length)];
  }

  useEffect(() => {
    init().then(() => {
      const canvas = canvasRef.current!;
      const context = canvas.getContext('2d');

      const circles = Array.from(
        { length: 200 },
        () =>
          new Circle(
            getRandomColor(),
            Math.random() * 15,
            new Coordinate(Math.random() * props.width, Math.random() * props.height),
            new Boundary(props.width, props.height)
          )
      );

      circles.forEach((circle) => {
        const randomTrueOrFalse = Math.floor(Math.random() * 2);
        let dx = Math.random();
        let dy = Math.random();

        if (randomTrueOrFalse === 1) {
          dx = -dx;
          dy = -dy;
        }
        circle.velocity = new Velocity(dx, dy);
      });

      const animate = () => {
        context!.clearRect(0, 0, props.width, props.height);
        circles.forEach((circle) => {
          circle.moving();
          circle.draw(context!);
        });
        requestAnimationFrame(animate);
      };

      requestAnimationFrame(animate);
    });
  }, [props.height, props.width]);

  return <canvas id="circle-canvas" ref={canvasRef} width={props.width} height={props.height} />;
};

export default CircleCanvas;
