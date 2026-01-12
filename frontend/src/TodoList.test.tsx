// @ts-nocheck
import { render, screen } from '@testing-library/react';
import TodoList from './TodoList';
import Todo from './TodoList'
import apiClient from './client';

const testFetchTodos = async () => {
  try {
    const response = await apiClient.get('/api/todos');
    return response.data;
  } catch (error) {
    console.error('Error fetching todos from API:', error);
  }
};

const testHandleAdd = async () => {
  const title = "test";

  try {
    const response = await apiClient.post<{ text: string }>('/api/todos', { title });
    return response.data;
  } catch (error) {
    console.error('Error adding todo remotely, falling back to local add:', error);
  }
};

const testToggleCompleted = async (id: number) => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/complete', { id });
  } catch (error) {
    console.error('Error toggling todo:', error);
  }
};

const testRenameTodo = async (id: number) => {
  const newTitle = "new_title";
  try {
    await apiClient.post<{ text: string }>('/api/todos/rename', { id, new_title: newTitle });
  } catch (error) {
    console.error('Error renaming todo:', error);
  }
};

const testDeleteTodo = async (id: number) => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/delete', { id });
  } catch (error) {
    console.error('Error deleting todo:', error);
  }
};

const testIncreaseTodoPriority = async (id: number) => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/increase_priority', { id });
  } catch (error) {
    console.error('Error increasing todo priority:', error);
  }
};

const testDecreaseTodoPriority = async (id: number) => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/decrease_priority', { id });
  } catch (error) {
    console.error('Error decreasing todo priority:', error);
  }
};

const testClearTodos = async () => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/clear', {});
  } catch (error) {
    console.error('Error clearing todos:', error);
  }
};

const testArchiveCompletedTodos = async () => {
  try {
    await apiClient.post<{ text: string }>('/api/todos/archive_completed', {});
  } catch (error) {
    console.error('Error archiving completed todos:', error);
  }
};

const testMarkAllCompleted = async (todos) => {
  try {
    const incompleteTodos = todos.filter((t) => !t.completed);
    for (const todo of incompleteTodos) {
      await apiClient.post<{ text: string }>('/api/todos/complete', { id: todo.id });
    }
  } catch (error) {
    console.error('Error marking all todos as completed:', error);
  }
};

const testShowArchivedTodos = async () => {
  try {
    const response = await apiClient.get('/api/todos/complete');
    return response.data;
  } catch (error) {
    console.error('Error fetching todos from API:', error);
  }
};

test('Renders the buttons correctly', () => {
  render(<TodoList />);
  const clear_all = screen.getByText(/Clear All/i);
  const archive_completed = screen.getByText(/Archive Completed/i);
  const add = screen.getByText(/Add/i);
  const show_archived_todos = screen.getByText(/Show Archived Todos/i);
  const mark_all_completed = screen.getByText(/Mark All Completed/i);
  expect(clear_all).toBeInTheDocument();
  expect(archive_completed).toBeInTheDocument();
  expect(add).toBeInTheDocument();
  expect(show_archived_todos).toBeInTheDocument();
  expect(mark_all_completed).toBeInTheDocument();
});

test('Renders text correctly', async () => {
  render(<TodoList />);
  const todo_list = await screen.findByText(/Todo List/i);
  const total_todos = await screen.findByText(/Total Todos:/i);
  expect(todo_list).toBeInTheDocument();
  expect(total_todos).toBeInTheDocument();
});

test('Todo can be added', async () => {
  const added_todo = await testHandleAdd();
  expect(added_todo.title).toEqual("test");
});

test('Todos can be fetched', async () => {
  await testHandleAdd();
  const fetched_todos = await testFetchTodos();
  expect(fetched_todos[fetched_todos.length - 1].title).toEqual("test");
});

test('Todos can be cleared', async () => {
  await testClearTodos();
  const fetched_todos = await testFetchTodos()
  expect(fetched_todos).toEqual([]);
});

test('Todos can be marked completed', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  const id = fetched_todos[0].id;
  await testToggleCompleted(id);
  fetched_todos = await testFetchTodos();
  expect(fetched_todos[0].completed).toEqual(true);
});

test('All todos can be marked completed', async () => {
  await testClearTodos();
  await testHandleAdd();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  await testMarkAllCompleted(fetched_todos);
  fetched_todos = await testFetchTodos();
  expect(fetched_todos[0].completed).toEqual(true);
  expect(fetched_todos[1].completed).toEqual(true);
});


test('Todo can be deleted', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testDeleteTodo(id);
  let new_todos = await testFetchTodos();
  expect(new_todos.length).toEqual(0);
});

test('Todo can be renamed', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testRenameTodo(id);
  let new_todos = await testFetchTodos();
  expect(new_todos[0].title).toEqual("new_title");
});

test('Todo priority can be increased', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testIncreaseTodoPriority(id);
  let new_todos = await testFetchTodos();
  expect(new_todos[0].priority).toEqual(fetched_todos[0].priority + 1);
});

test('Todo priority can be decreased', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testDecreaseTodoPriority(id);
  let new_todos = await testFetchTodos();
  expect(new_todos[0].priority).toEqual(fetched_todos[0].priority - 1);
});

test('Completed todos can be archived', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testToggleCompleted(id)
  await testArchiveCompletedTodos()
  let new_todos = await testFetchTodos();
  expect(new_todos.length).toEqual(0);

});

test('Archived todos can be fetched', async () => {
  await testClearTodos();
  await testHandleAdd();
  let fetched_todos = await testFetchTodos();
  let id = fetched_todos[0].id;
  await testToggleCompleted(id)
  await testArchiveCompletedTodos()
  let archived_todos = await testShowArchivedTodos()
  expect(archived_todos.length).not.toEqual(0);
});
