import {
  useQuery,
  useMutation,
  type UseMutationOptions,
  type UseQueryOptions
} from '@sveltestack/svelte-query';

import { queryClient } from './queryClient';

type Todo = {
  id: string;
  description: string;
  done: boolean;
  createdAt: string;
  doneAt: string;
  kind?: string;
};

type ListTodos = {
  informations: {
    total: number;
  };
  data: Todo[];
  status: string;
};

export function useTodosQuery(
  args: { status: string } = { status: 'all' },
  queryOptions: UseQueryOptions<ListTodos, unknown, ListTodos, ['todos', string]> = {}
) {
  return useQuery(
    ['todos', args.status],
    ({ queryKey }) => {
      const [, status] = queryKey;

      return fetch(`http://localhost:3100/api/todos?status=${status}`).then((res) => res.json());
    },
    {
      initialData: { data: [], informations: { total: 0 }, status: '' },
      refetchOnWindowFocus: false,
      ...queryOptions
    }
  );
}

export function useTodoDoneMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { id: string }, unknown> = {}
) {
  return useMutation(
    (data: { id: string }) => {
      return fetch(`http://localhost:3100/api/todos/${data.id}/mark_as_done`, {
        method: 'PATCH'
      })
        .then((res) => res.json())
        .then((res) => res?.data);
    },
    {
      mutationKey: 'todo-done',
      onSuccess: (updateTodo) => {
        const status = new URLSearchParams(window.location.search).get('status') || 'all';

        if (status === 'pending') {
          queryClient.setQueryData<ListTodos>(['todos', status], (old) =>
            updateListTodoCache(old, updateTodo, 'REMOVE')
          );
          return;
        }

        queryClient.setQueryData<ListTodos>(['todos', status], (old) =>
          updateListTodoCache(old, updateTodo, 'UPDATE')
        );
      },
      ...queryOptions
    }
  );
}

export function useTodoUndoneMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { id: string }, unknown> = {}
) {
  return useMutation(
    (data: { id: string }) =>
      fetch(`http://localhost:3100/api/todos/${data.id}/mark_as_undone`, {
        method: 'PATCH'
      })
        .then((res) => res.json())
        .then((res) => res?.data),
    {
      mutationKey: 'todo-undone',
      onSuccess: (updateTodo) => {
        const status = new URLSearchParams(window.location.search).get('status') || 'all';

        if (status === 'done') {
          queryClient.setQueryData<ListTodos>(['todos', status], (old) =>
            updateListTodoCache(old, updateTodo, 'REMOVE')
          );

          return;
        }

        queryClient.setQueryData<ListTodos>(['todos', status], (old) =>
          updateListTodoCache(old, updateTodo, 'UPDATE')
        );
      },
      ...queryOptions
    }
  );
}

export function useTodoRemoveMutation(
  queryOptions: UseMutationOptions<void, unknown, { id: string }, unknown> = {}
) {
  return useMutation(
    async (data: { id: string }) => {
      await fetch(`http://localhost:3100/api/todos/${data.id}`, {
        method: 'DELETE'
      });
    },
    {
      mutationKey: 'todo-remove',
      onSuccess: async (_, variables) => {
        const status = new URLSearchParams(window.location.search).get('status') || 'all';

        queryClient.setQueryData<ListTodos>(['todos', status], (old) =>
          updateListTodoCache(old, { id: variables.id } as Todo, 'REMOVE')
        );
      },
      ...queryOptions
    }
  );
}

export function useTodoCreateMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { description: string }, unknown> = {}
) {
  return useMutation(
    async (data: { description: string }) => {
      return fetch('http://localhost:3100/api/todos', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
          'Content-Type': 'application/json'
        }
      })
        .then((res) => res.json())
        .then((res) => res?.data);
    },
    {
      mutationKey: 'todo-create',
      onSuccess: async (newTodo) => {
        queryClient.setQueryData<ListTodos>(['todos', 'all'], (old) =>
          updateListTodoCache(old, newTodo, 'ADD')
        );
        queryClient.setQueryData<ListTodos>(['todos', 'pending'], (old) =>
          updateListTodoCache(old, newTodo, 'ADD')
        );
      },
      ...queryOptions
    }
  );
}

function updateListTodoCache(
  old: ListTodos | undefined,
  todo: Todo | { id: string },
  action: 'ADD' | 'REMOVE' | 'UPDATE'
) {
  let data = old?.data || [];
  let increaseTotal = 0;

  if (action === 'ADD' && isTodo(todo)) {
    data = [todo].concat(data);
    increaseTotal = 1;
  }

  if (action === 'REMOVE') {
    data = data.filter((item) => item.id !== todo.id);
    increaseTotal = -1;
  }

  if (action === 'UPDATE' && isTodo(todo)) {
    data = data.map((item) => (item.id === todo.id ? todo : item));
  }

  return {
    ...old,
    data,
    status: old?.status ?? '',
    informations: {
      ...(old?.informations || {}),
      total: old?.informations?.total ? old.informations.total + increaseTotal : 0
    }
  };
}

function isTodo(todo: Todo | { id: string }): todo is Todo {
  return (todo as Todo).done !== undefined;
}
