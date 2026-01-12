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

  useEffect(() => {
    const fetchTodos = async () => {
      try {
        const response = await apiClient.get('/api/todos');
        setTodos(response.data);
      } catch (error) {
        console.error('Error fetching todos from API:', error);
      }
    };

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
      await apiClient.post<Todo>('/api/todos/complete', { id });
      setTodos((prev) => prev.map((t) => t.id === id ? { ...t, completed: !t.completed } : t));
    } catch (error) {
      console.error('Error toggling todo:', error);
    }
  };

  const renameTodo = (id: number) => async () => {
    const newTitle = prompt('Enter new title for the todo:');
    if (!newTitle) return;
    if (newTitle.trim() === '') return;
    if (newTitle.length > 22) {
      alert('Title cannot be longer than 22 characters.');
      return;
    }
    console.log('Renaming todo with id:', id, 'to new title:', newTitle);
    try {
      await apiClient.post<Todo>('/api/todos/rename', { id, new_title: newTitle });
      setTodos((prev) => prev.map((t) => t.id === id ? { ...t, title: newTitle } : t));
    } catch (error) {
      console.error('Error renaming todo:', error);
    }
  };

  const deleteTodo = (id: number) => async () => {
    console.log('Deleting todo with id:', id);
    try {
      await apiClient.post<{ text: string }>('/api/todos/delete', { id });
      setTodos((prev) => prev.filter((t) => t.id !== id));
    } catch (error) {
      console.error('Error deleting todo:', error);
    }
  };

  const increaseTodoPriority = (id: number) => async () => {
    if (todos.find((t) => t.id === id)?.priority === 10 ||
      todos.find((t) => t.id === id)?.completed === true) return;
    console.log('Increasing priority for todo with id:', id);
    try {
      await apiClient.post<{ text: string }>('/api/todos/increase_priority', { id });
      setTodos((prev) => prev.map((t) => t.id === id ? { ...t, priority: t.priority + 1 } : t));
    } catch (error) {
      console.error('Error increasing todo priority:', error);
    }
  };

  const decreaseTodoPriority = (id: number) => async () => {
    if (todos.find((t) => t.id === id)?.priority === 1 ||
      todos.find((t) => t.id === id)?.completed === true) return;
    console.log('Decreasing priority for todo with id:', id);
    try {
      await apiClient.post<{ text: string }>('/api/todos/decrease_priority', { id });
      setTodos((prev) => prev.map((t) => t.id === id ? { ...t, priority: t.priority - 1 } : t));
    } catch (error) {
      console.error('Error decreasing todo priority:', error);
    }
  };

  const clearTodos = async () => {
    try {
      await apiClient.post<{ text: string }>('/api/todos/clear', {});
      setTodos([]);
    } catch (error) {
      console.error('Error clearing todos:', error);
    }
  };

  const archiveCompletedTodos = async () => {
    try {
      await apiClient.post<{ text: string }>('/api/todos/archive_completed', {});
      setTodos((prev) => prev.filter((t) => !t.completed));
    } catch (error) {
      console.error('Error archiving completed todos:', error);
    }
  };

  const markAllCompleted = async () => {
    try {
      const incompleteTodos = todos.filter((t) => !t.completed);
      for (const todo of incompleteTodos) {
        await apiClient.post<Todo>('/api/todos/complete', { id: todo.id });
      }
      setTodos((prev) => prev.map((t) => ({ ...t, completed: true })));
    } catch (error) {
      console.error('Error marking all todos as completed:', error);
    }
  };

  const showArchivedTodos = async () => {
    try {
      const response = await apiClient.get('/api/todos/complete');
      let completedTodos = response.data;
      alert(`Completed Todos:\n${completedTodos.map((t: Todo) => t.title).join('\n')}`);
    } catch (error) {
      console.error('Error fetching todos from API:', error);
    }
  };

  const noBullets = {
    listStyleType: 'none'
  }

  return (
    <div>
      <h1>Todo List</h1>
      <div className="todo-count" style={{ marginBottom: 20, fontWeight: 'bold' }}>
        Total Todos: {todos.length}
      </div>
      <div style={{ marginBottom: 14, display: 'flex', gap: 8 }}>
        <button
          onClick={archiveCompletedTodos}
          style={{
            padding: '8px 16px',
            borderRadius: 4,
            border: 'none',
            backgroundColor: '#6c757d',
            color: 'white',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 500,
          }}
        >
          Archive Completed
        </button>
        <input
          type="text"
          maxLength={22}
          placeholder="Add todo..."
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          onKeyDown={async (e) => {
            if (e.key === 'Enter') {
              await handleAdd();
            }
          }}
          style={{
            accentColor: '#007bff',
            flex: 2,
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
        <button
          onClick={clearTodos}
          style={{
            padding: '8px 16px',
            borderRadius: 4,
            border: 'none',
            backgroundColor: '#148a18',
            color: 'white',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 500,
          }}
        >
          Clear All
        </button>
      </div>
      <div>
        <button
          onClick={showArchivedTodos}
          style={{
            padding: '8px 16px',
            marginRight: 8,
            borderRadius: 4,
            border: 'none',
            backgroundColor: '#e0960d',
            color: 'white',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 500,
          }}
        >
          Show Archived Todos
        </button>
        <button
          onClick={markAllCompleted}
          style={{
            padding: '8px 16px',
            borderRadius: 4,
            border: 'none',
            backgroundColor: '#242ec7',
            color: 'white',
            cursor: 'pointer',
            fontSize: 14,
            fontWeight: 500,
          }}
        >
          Mark All Completed
        </button>

      </div>
      <ul style={noBullets}>
        {todos.sort((a, b) => b.priority - a.priority || a.id - b.id).map((todo) => (
          <li style={{
            marginBottom: 8,
            display: 'flex',
            alignItems: 'center',
            width: '100%'
          }}
            key={todo.id}>
            <span style={{
              textDecoration: 'none',
              color: todo.completed ? 'gray' : 'white', marginLeft: 9
            }}>
              {todo.priority}.
            </span>
            <span style={{
              textDecoration: todo.completed ? 'line-through' : 'none',
              color: todo.completed ? 'gray' : 'white', marginLeft: 8,
            }}>
              {todo.title}
            </span>
            <div style={{ marginLeft: 'auto', display: 'flex', alignItems: 'center' }}>
              <button
                onClick={deleteTodo(todo.id)}
                style={{
                  marginLeft: 80,
                  marginRight: 4,
                  padding: '4px 8px',
                  borderRadius: 4,
                  border: 'none',
                  backgroundColor: todo.completed ? '#ff0019' : '#9a323d',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: 12,
                }}
              >
                Delete
              </button>
              <button
                onClick={renameTodo(todo.id)}
                style={{
                  marginLeft: 4,
                  marginRight: 4,
                  padding: '4px 8px',
                  borderRadius: 4,
                  border: 'none',
                  backgroundColor: todo.completed ? '#827724' : '#ffe100',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: 12,
                }}
              >
                Rename
              </button>
              <button
                onClick={increaseTodoPriority(todo.id)}
                style={{
                  marginRight: 4,
                  padding: '1px 6px',
                  borderRadius: 4,
                  border: 'none',
                  backgroundColor: todo.completed ? '#202e23' : '#28a745',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: 12,
                }}
              >
                +
              </button>
              <button
                onClick={decreaseTodoPriority(todo.id)}
                style={{
                  marginRight: 4,
                  padding: '1px 6px',
                  borderRadius: 4,
                  border: 'none',
                  backgroundColor: todo.completed ? '#40371c' : '#ffc107',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: 12,
                }}
              >
                -
              </button>
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleCompleted(todo.id)}
              />
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TodoList;
