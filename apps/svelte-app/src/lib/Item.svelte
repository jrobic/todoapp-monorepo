<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Todo } from './types/todo';

  export let todo: Todo;

  const dispatch = createEventDispatcher();
</script>

<div
  id={todo.id}
  class="flex gap-4 py-1 cursor-pointer text-lg dark:hover:bg-slate-600 hover:bg-slate-100"
  data-type="item"
>
  <div class="flex items-center gap-2 flex-1">
    <span class={todo.done ? 'text-pink-400' : 'text-blue-400 accent-current'}>
      {todo.description}
    </span>
  </div>

  <div class="grid grid-cols-2 items-center gap-2">
    <span class="text-xs text-gray-300"> {new Date(todo.createdAt).toLocaleString()} </span>
    <span class="text-xs text-green-400">
      {todo.doneAt ? new Date(todo.doneAt).toLocaleString() : ''}
    </span>
  </div>

  <div class="flex gap-1">
    <button
      type="button"
      data-action={todo.done ? 'todo-undone' : 'todo-done'}
      class="btn btn-circle btn-sm btn-ghost"
      class:hover:bg-fuchsia-400={todo.done}
      class:hover:bg-teal-400={!todo.done}
      on:click={() => dispatch(todo.done ? 'undone' : 'done', todo.id)}
    >
      {todo.done ? '✘' : '✔︎'}
    </button>
    <button
      type="button"
      data-action="todo-remove"
      class="btn btn-circle btn-sm btn-ghost hover:bg-red-400"
      on:click={() => dispatch('remove', todo.id)}
    >
      🗑
    </button>
  </div>
</div>
