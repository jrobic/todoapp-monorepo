/* eslint-disable react/destructuring-assignment */
import clsx from 'clsx';
import { ReactElement } from 'react';

export type TodoProps = {
  id: string;
  description: string;
  done: boolean;
  createdAt: string;
  doneAt: string;
  handleRemove?: () => void;
  handleDone?: () => void;
  handleUndone?: () => void;
};

export default function Item(todo: TodoProps): ReactElement {
  return (
    <div
      id={todo.id}
      className="flex gap-4 py-1 cursor-pointer text-lg dark:hover:bg-slate-600 hover:bg-slate-100"
      data-type="item"
    >
      <div className="flex items-center gap-2 flex-1">
        <span className={todo.done ? 'text-pink-400' : 'text-blue-400 accent-current'}>
          {todo.description}
        </span>
      </div>

      <div className="grid grid-cols-2 items-center gap-2">
        <span className="text-xs text-gray-300"> {new Date(todo.createdAt).toLocaleString()} </span>
        <span className="text-xs text-green-400">
          {' '}
          {todo.doneAt ? new Date(todo.doneAt).toLocaleString() : null}{' '}
        </span>
      </div>

      <div className="flex gap-1">
        <button
          type="button"
          data-action={todo.done ? 'todo-undone' : 'todo-done'}
          className={clsx([
            'btn btn-circle btn-sm btn-ghost',
            todo.done ? 'hover:bg-fuchsia-400' : 'hover:bg-teal-400',
          ])}
          onClick={todo.done ? todo.handleUndone : todo.handleDone}
        >
          {todo.done ? '✘' : '✔︎'}
        </button>
        <button
          type="button"
          data-action="todo-remove"
          className="btn btn-circle btn-sm btn-ghost hover:bg-red-400"
          onClick={todo.handleRemove}
        >
          🗑
        </button>
      </div>
    </div>
  );
}
