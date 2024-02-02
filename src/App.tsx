import { useEffect, useState } from 'react';
import CircleCanvas from './components/CircleCanvas';
function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width: width - 8,
    height: height - 8,
  };
}

function App() {
  const [windowDimensions, setWindowDimensions] = useState(getWindowDimensions());

  useEffect(() => {
    function handleResize() {
      setWindowDimensions(getWindowDimensions());
    }

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return <CircleCanvas width={windowDimensions.width} height={windowDimensions.height} />;
}

export default App;
