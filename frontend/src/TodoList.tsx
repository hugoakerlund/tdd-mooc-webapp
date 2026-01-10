import React, { useState, useEffect } from 'react';
import apiClient from './client';

interface Todo {
  id: number;
  priority: number;
  title: string;
  completed: boolean;
}

const TodoList: React.FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTitle, setNewTitle] = useState('');
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

    const fetchTodos = async () => {
      try {
        const response = await apiClient.get('/api/todos');
        setTodos(response.data);
      } catch (error) {
        console.error('Error fetching todos from API:', error);
      }
    };

    fetchMessage();
    fetchTodos();

  }, []);



  const handleAdd = async () => {
    const title = newTitle.trim();
    if (!title) return;

    try {
      const response = await apiClient.post<Todo>('/api/todos', { title });
      setTodos((prev) => [...prev, response.data]);
    } catch (error) {
      console.log(error);
      console.error('Error adding todo remotely, falling back to local add:', error);
      const fallback: Todo = { id: Date.now(), priority: 0, title, completed: false };
      setTodos((prev) => [...prev, fallback]);
    }

    setNewTitle('');
  };

  const toggleCompleted = async (id: number) => {
    console.log('Toggling todo with id:', id);
    const todo = todos.find((t) => t.id === id);
    if (!todo) return;
    try {
      const response = await apiClient.post<Todo>('/api/todos/complete', { id });
      setTodos((prev) => prev.map((t) => t.id === id ? { ...t, completed: !t.completed } : t));
    } catch (error) {
      console.error('Error toggling todo:', error);
    }
  };

  return (
    <div>
      <h1>Todo List</h1>
      <div className="todo-count" style={{ marginBottom: 20, fontWeight: 'bold' }}>
        Total Todos: {todos.length}
      </div>
      <div style={{ marginBottom: 20 }}>
        {apiMessage ? (apiMessage) : (loading ? 'Loading message from API...' : 'No message from API')}
      </div>
      <div style={{ marginBottom: 14, display: 'flex', gap: 8 }}>
        <input
          type="text"
          placeholder="Add todo..."
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          onKeyDown={async (e) => {
            if (e.key === 'Enter') {
              await handleAdd();
            }
          }}
          style={{
            flex: 1,
            padding: '8px 12px',
            borderRadius: 4,
            border: '1px solid #ddd',
            fontSize: 14,
            fontFamily: 'inherit',
          }}
        />
        <button
          onClick={handleAdd}
          style={{
            padding: '8px 16px',
            borderRadius: 4,
            border: 'none',
            backgroundColor: '#007bff',
            color: 'white',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 500,
          }}
        >
          Add
        </button>
      </div>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id} style={{ marginBottom: 8 }}>
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => toggleCompleted(todo.id)}
            />
            <span style={{ textDecoration: todo.completed ? 'line-through' : 'none', marginLeft: 8 }}>
              {todo.priority}
            </span>
            <span style={{ textDecoration: todo.completed ? 'line-through' : 'none', marginLeft: 8 }}>
              {todo.title}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TodoList;