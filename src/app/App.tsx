import './styles/index.css';
import useLoadColors from './hooks/useLoadColors';
import useNeuralSetup from './hooks/useNeuralSetup';
import useLoadTheme from './hooks/useLoadTheme';
import AppBar from '@/components/AppBar/AppBar';
import Outlet from '@/components/Outlet/Outlet';
import { PageProvider } from './contexts/PageContext';

function App() {
  useLoadColors();
  useNeuralSetup();
  useLoadTheme();

  return (
    <PageProvider>
      <AppBar />
      <main>
        <Outlet />
      </main>
    </PageProvider>
  );
}

export default App;
