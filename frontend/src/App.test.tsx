import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('Renders the title', () => {
  render(<App />);
  const linkElement = screen.getByText(/Todo List/i);
  expect(linkElement).toBeInTheDocument();
});
