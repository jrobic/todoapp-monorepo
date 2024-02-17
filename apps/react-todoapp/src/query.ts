import { useQuery, useMutation, UseMutationOptions, UseQueryOptions } from 'react-query';

import { queryClient } from './client';

type Todo = {
  id: string;
  description: string;
  done: boolean;
  createdAt: string;
  doneAt: string;
  kind?: string;
};

export function useTodosQuery(
  args: { status: string } = { status: 'all' },
  queryOptions: UseQueryOptions<Todo[], unknown, Todo[], ['todos', string]> = {},
) {
  return useQuery(
    ['todos', args.status],
    ({ queryKey }) => {
      const [, status] = queryKey;

      return fetch(`http://localhost:3000/api/todos?status=${status}`)
        .then((res) => res.json())
        .then((data) => data?.data);
    },
    { initialData: [], refetchOnWindowFocus: false, ...queryOptions },
  );
}

export function useTodoDoneMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { id: string }, unknown> = {},
) {
  return useMutation({
    mutationFn: (data: { id: string }) =>
      fetch(`http://localhost:3000/api/todos/${data.id}/mark_as_done`, {
        method: 'PATCH',
      })
        .then((res) => res.json())
        .then((res) => res?.data),
    mutationKey: 'todo-done',
    onSuccess: async (updateTodo, variables) => {
      const status = new URLSearchParams(window.location.search).get('status') || 'all';

      if (status === 'pending') {
        queryClient.setQueryData<Todo[]>(['todos', status], (old) =>
          (old || []).filter((todo) => todo.id !== variables.id),
        );
        return;
      }

      queryClient.setQueryData<Todo[]>(['todos', status], (old) =>
        (old || []).map((todo) => (todo.id === variables.id ? updateTodo : todo)),
      );

      queryClient.setQueryData<number>(['todos-count', 'done'], (old) => (old || 0) + 1);
      queryClient.setQueryData<number>(['todos-count', 'pending'], (old) => (old || 0) - 1);
    },
    ...queryOptions,
  });
}

export function useTodoUndoneMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { id: string }, unknown> = {},
) {
  return useMutation({
    mutationFn: (data: { id: string }) =>
      fetch(`http://localhost:3000/api/todos/${data.id}/mark_as_undone`, {
        method: 'PATCH',
      })
        .then((res) => res.json())
        .then((res) => res?.data),
    mutationKey: 'todo-undone',
    onSuccess: async (updateTodo, variables) => {
      const status = new URLSearchParams(window.location.search).get('status') || 'all';

      if (status === 'done') {
        queryClient.setQueryData<Todo[]>(['todos', status], (old) =>
          (old || []).filter((todo) => todo.id !== variables.id),
        );

        return;
      }

      queryClient.setQueryData<Todo[]>(['todos', status], (old) =>
        (old || []).map((todo) => (todo.id === variables.id ? updateTodo : todo)),
      );

      queryClient.setQueryData<number>(['todos-count', 'done'], (old) => (old || 0) - 1);
      queryClient.setQueryData<number>(['todos-count', 'pending'], (old) => (old || 0) + 1);
    },
    ...queryOptions,
  });
}

export function useTodoRemoveMutation(
  queryOptions: UseMutationOptions<void, unknown, { id: string }, unknown> = {},
) {
  return useMutation({
    mutationFn: async (data: { id: string }) => {
      await fetch(`http://localhost:3000/api/todos/${data.id}`, {
        method: 'DELETE',
      });
    },
    mutationKey: 'todo-remove',
    onSuccess: async (_, variables) => {
      const status = new URLSearchParams(window.location.search).get('status') || 'all';

      queryClient.setQueryData<Todo[]>(['todos', status], (old) =>
        (old || []).filter((todo) => todo.id !== variables.id),
      );

      if (status === 'all' || status === 'pending') {
        queryClient.setQueryData<number>(['todos-count', 'all'], (old) => (old || 0) - 1);
        queryClient.setQueryData<number>(['todos-count', 'pending'], (old) => (old || 0) - 1);
      } else {
        queryClient.setQueryData<number>(['todos-count', 'done'], (old) => (old || 0) - 1);
      }
    },
    ...queryOptions,
  });
}

export function useTodoCreateMutation(
  queryOptions: UseMutationOptions<Todo, unknown, { description: string }, unknown> = {},
) {
  return useMutation({
    mutationFn: async (data: { description: string }) => {
      return fetch('http://localhost:3000/api/todos', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
          'Content-Type': 'application/json',
        },
      })
        .then((res) => res.json())
        .then((res) => res?.data);
    },
    mutationKey: 'todo-create',
    onSuccess: async (newTodo) => {
      queryClient.setQueryData<Todo[]>(['todos', 'all'], (old) => [newTodo].concat(old || []));
      queryClient.setQueryData<Todo[]>(['todos', 'pending'], (old) => [newTodo].concat(old || []));
      queryClient.setQueryData<number>(['todos-count', 'all'], (old) => (old || 0) + 1);
      queryClient.setQueryData<number>(['todos-count', 'pending'], (old) => (old || 0) + 1);
    },
    ...queryOptions,
  });
}

export function useTodosCountQuery(
  args: { status: string } = { status: 'all' },
  queryOptions: UseQueryOptions<number, unknown, number, ['todos-count', string]> = {},
) {
  return useQuery(
    ['todos-count', args.status],
    ({ queryKey }) => {
      const [, status] = queryKey;

      return fetch(`http://localhost:3000/api/todos/count?status=${status}`)
        .then((res) => res.json())
        .then((data) => data?.data);
    },
    { initialData: 0, refetchOnWindowFocus: false, ...queryOptions },
  );
}

export function refreshTodosList() {
  queryClient.invalidateQueries('todos');
  queryClient.invalidateQueries('todos-count');
}
