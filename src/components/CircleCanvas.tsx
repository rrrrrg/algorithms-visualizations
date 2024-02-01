import React, { useEffect, useRef } from 'react';
import init, { Circle, Velocity } from 'wasm-libs';

interface Props {
  width: number;
  height: number;
}
const CircleCanvas: React.FunctionComponent<Props> = (props) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  function getRandomColor(): string {
    const COLORS = ['#272F32', '#9DBDC6', '#FF3D2E', '#DAEAEF'];

    return COLORS[Math.floor(Math.random() * COLORS.length)];
  }

  useEffect(() => {
    init().then(() => {
      const canvas = canvasRef.current!;
      const context = canvas.getContext('2d');
      const circle = new Circle('black', 100, 100, 50);
      circle.moving(new Velocity(2, 2));
      circle.draw(context!);

      const circles = Array.from(
        { length: 100 },
        () => new Circle(getRandomColor(), Math.random() * 100, Math.random() * 100, 50)
      );

      const animate = () => {
        context!.clearRect(0, 0, canvas.width, canvas.height);
        circle.moving(new Velocity(Math.random() * 3, Math.random() * 3));
        circle.draw(context!);
        circles.forEach((circle) => {
          circle.moving(new Velocity(Math.random() * 3, Math.random() * 5));
          circle.draw(context!);
        });
        requestAnimationFrame(animate);
      };

      requestAnimationFrame(animate);
    });
  }, []);

  return <canvas id='circle-canvas' ref={canvasRef} width={props.width} height={props.height} />;
};

export default CircleCanvas;
