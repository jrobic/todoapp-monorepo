import clsx from 'clsx';
import { ReactElement } from 'react';

export type FooterProps = {
  todosCount: number | undefined;
  status: string;
  handleStatus: (status: string) => void;
};

export function Footer({ todosCount, handleStatus, status }: FooterProps): ReactElement {
  return (
    <footer className="flex items-center p-4 justify-between">
      <span id="todo-count">
        <strong>{todosCount || 0}</strong> items left
      </span>

      <ul className="flex gap-4">
        <li>
          <button
            className={clsx(['link hover:text-blue-400', status === 'all' && 'text-blue-400'])}
            data-status="all"
            onClick={() => handleStatus('all')}
            type="button"
          >
            All
          </button>
        </li>
        <li>
          <button
            className={clsx(['link hover:text-blue-400', status === 'pending' && 'text-blue-400'])}
            data-status="pending"
            type="button"
            onClick={() => handleStatus('pending')}
          >
            Active
          </button>
        </li>
        <li>
          <button
            className={clsx(['link hover:text-blue-400', status === 'done' && 'text-blue-400'])}
            data-status="done"
            type="button"
            onClick={() => handleStatus('done')}
          >
            Completed
          </button>
        </li>
      </ul>

      <div />
    </footer>
  );
}
