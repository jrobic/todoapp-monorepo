import { ReactElement } from 'react';

import Item, { TodoProps } from './Item';

export type ListProps = {
  todos: TodoProps[];
  handleRemove?: (id: string) => void;
  handleDone?: (id: string) => void;
  handleUndone?: (id: string) => void;
};

export function List({ todos, handleDone, handleUndone, handleRemove }: ListProps): ReactElement {
  return (
    <ul id="list-todos">
      {todos.map((todo) => (
        <li key={todo.id}>
          <Item
            id={`item-${todo.id}`}
            description={todo.description}
            done={todo.done}
            createdAt={todo.createdAt}
            doneAt={todo.doneAt}
            handleDone={() => handleDone?.(todo.id)}
            handleUndone={() => handleUndone?.(todo.id)}
            handleRemove={() => handleRemove?.(todo.id)}
          />
        </li>
      ))}
    </ul>
  );
}
