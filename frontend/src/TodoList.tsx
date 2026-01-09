import React, { useState, useEffect } from 'react';
import axios from 'axios';

interface Todo {
  id: number;
  priority: number;
  title: string;
  completed: boolean;
}

const TodoList: React.FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTitle, setNewTitle] = useState('');

  useEffect(() => {
    const fetchTodos = async () => {
      try {
        const response = await axios.get<Todo[]>('/api/todos');
        setTodos(response.data);
      } catch (error) {
        console.error('Error fetching todos:', error);
      }
    };

    fetchTodos();
  }, []);

  const handleAdd = async () => {
    const title = newTitle.trim();
    if (!title) return;

    try {
      const response = await axios.post<Todo>('/api/todos', { title });
      setTodos((prev) => [...prev, response.data]);
    } catch (error) {
      console.error('Error adding todo remotely, falling back to local add:', error);
      const fallback: Todo = { id: Date.now(), priority: 0, title, completed: false };
      setTodos((prev) => [...prev, fallback]);
    }

    setNewTitle('');
  };

  const toggleCompleted = async (id: number) => {
    setTodos((prev) => {
      const updated = prev.map((t) => (t.id === id ? { ...t, completed: !t.completed } : t));
      const newVal = updated.find((t) => t.id === id)!.completed;

      (async () => {
        try {
          await axios.patch(`/api/todos/${id}`, { completed: newVal });
        } catch (error) {
          console.error('Failed to update todo remotely, reverting:', error);
          setTodos((curr) => curr.map((t) => (t.id === id ? { ...t, completed: !newVal } : t)));
        }
      })();

      return updated;
    });
  };

  return (
    <div>
      <h1>Todo List</h1>
      <div style={{ marginBottom: 12 }}>
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
        />
        <button onClick={handleAdd} style={{ marginLeft: 8 }}>
          Add
        </button>
      </div>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            <input
              type="checkbox"
              checked={todo.completed}
              onChange={() => toggleCompleted(todo.id)}
            />
            {todo.title}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TodoList;