import TodoList from './TodoList';
import apiClient from './client';
import logo from './logo.svg';
import './App.css';
import { useEffect, useState } from 'react';

function App() {
  const [apiMessage, setApiMessage] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchMessage = async () => {
      try {
        const response = await apiClient.get('/');
        setApiMessage(response.data.text);
      }
      catch (error) {
        console.error('Error fetching message from API:', error);
        setApiMessage('Error fetching message');
      }
      finally {
        setLoading(false);
      }
    };
    fetchMessage();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <div className="api-message">
          {loading ? 'Loading message from API...' : apiMessage}
        </div>
        <TodoList />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
      </header>
      <main>  
        <TodoList />
      </main>
    </div>
  );
}

export default App;
