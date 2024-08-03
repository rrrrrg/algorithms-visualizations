import { useEffect, useState } from 'react';
import CircleCanvas from './components/CircleCanvas';
import GraphCanvas from './components/GraphCanvas';

function getWindowDimensions() {
  const { innerWidth: width, innerHeight: height } = window;
  return {
    width: width - 20,
    height: height - 25,
  };
}

function App() {
  const [windowDimensions, setWindowDimensions] = useState(getWindowDimensions());
  const [selectedComponent, setSelectedComponent] = useState('circle');

  useEffect(() => {
    function handleResize() {
      setWindowDimensions(getWindowDimensions());
    }

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  const handleSidebarClick = (component: string) => {
    setSelectedComponent(component);
  };

  const renderComponent = () => {
    if (selectedComponent === 'circle') {
      return <CircleCanvas width={windowDimensions.width} height={windowDimensions.height} />;
    } else if (selectedComponent === 'square') {
      return <GraphCanvas width={windowDimensions.width} height={windowDimensions.height} />;
    }
    // Add more conditions for other components if needed
  };

  return (
    <div>
      <div>
        <button onClick={() => handleSidebarClick('circle')}>Circle</button>
        <button onClick={() => handleSidebarClick('square')}>Square</button>
        {/* Add more buttons for other components if needed */}
      </div>
      {renderComponent()}
    </div>
  );
}

export default App;
