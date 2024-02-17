import clsx from 'clsx';
import { useCallback, useState } from 'react';

import { Footer } from './components/Footer';
import { List } from './components/List';
import {
  useTodosCountQuery,
  useTodoCreateMutation,
  useTodoDoneMutation,
  useTodoRemoveMutation,
  useTodoUndoneMutation,
  useTodosQuery,
  refreshTodosList,
} from './query';

function App() {
  const [status, setStatus] = useState(() => {
    return new URLSearchParams(window.location.search).get('status') || 'all';
  });

  const { isLoading, data, isRefetching } = useTodosQuery(
    { status },
    {
      enabled: !!status,
    },
  );
  const todosCountQuery = useTodosCountQuery(
    { status },
    {
      enabled: !!status,
    },
  );

  const todoDoneMutation = useTodoDoneMutation();
  const todoUndoneMutation = useTodoUndoneMutation();
  const todoRemoveMutation = useTodoRemoveMutation();

  const todoCreateMutation = useTodoCreateMutation();

  const handleTodoDone = useCallback(
    async (id: string) => {
      await todoDoneMutation.mutateAsync({ id });
    },
    [todoDoneMutation],
  );

  const handleTodoUndone = useCallback(
    async (id: string) => {
      await todoUndoneMutation.mutateAsync({ id });
    },
    [todoUndoneMutation],
  );

  const handleTodoRemove = useCallback(
    async (id: string) => {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Are you sure you want to delete this todo?')) return;

      await todoRemoveMutation.mutateAsync({ id });
    },
    [todoRemoveMutation],
  );

  const handleTodoCreate = useCallback(
    async (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();

      const form = event.target as HTMLFormElement;
      const description = form.description.value;

      await todoCreateMutation.mutateAsync({ description });

      form.reset();
    },
    [todoCreateMutation],
  );

  const handleStatus = (newStatus: string) => {
    window.history.replaceState(null, '', `?status=${newStatus}`);
    setStatus(newStatus);
  };

  return (
    <div className="grid grid-rows-layout h-screen flex-1">
      <div className="">
        <header className="px-4 py-8">
          <h1>
            <span className="text-transparent font-extrabold text-5xl bg-gradient-to-r bg-clip-text from-pink-500 to-blue-500">
              Todo App
            </span>
          </h1>

          <div className="pt-8 flex justify-center items-center gap-4">
            <form className="flex-1" id="new-todo" onSubmit={handleTodoCreate}>
              <input
                type="text"
                name="description"
                placeholder="What needs to be done?"
                className="input input-bordered w-full"
                required
                minLength={3}
              />
            </form>
            <button
              className="btn btn-square"
              onClick={() => refreshTodosList()}
              type="button"
              aria-label="Refresh todos list"
              disabled={isLoading || isRefetching}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={1.5}
                stroke="currentColor"
                className={clsx(['w-6 h-6', isRefetching && 'animate-spin'])}
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
                />
              </svg>
            </button>
          </div>
        </header>
      </div>

      <div className="overflow-auto">
        <main className="px-4">
          <List
            todos={data || []}
            handleDone={handleTodoDone}
            handleUndone={handleTodoUndone}
            handleRemove={handleTodoRemove}
          />
        </main>
      </div>

      <div className="">
        <Footer todosCount={todosCountQuery?.data} handleStatus={handleStatus} status={status} />
      </div>
    </div>
  );
}

export default App;
