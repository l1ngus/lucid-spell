/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import './styles/index.css';
import useNeuralSetup from './hooks/useNeuralSetup';
import useLoadTheme from './hooks/useLoadTheme';
import useShowWindow from './hooks/useShowWindow';
import AppBar from '@/components/AppBar/AppBar';
import Outlet from '@/components/Outlet/Outlet';
import { PageProvider } from './contexts/PageContext';
import useProxySetup from './hooks/useProxySetup';

function App() {
  useNeuralSetup();
  useLoadTheme();
  useShowWindow();
  useProxySetup();

  return (
    <PageProvider>
      <AppBar />
      <main className='flex flex-col flex-1' >
        <Outlet />
      </main>
    </PageProvider>
  );
}

export default App;
