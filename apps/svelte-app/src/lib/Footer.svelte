<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let todosCount: number = 0;
  export let status: string | undefined;

  const dispatch = createEventDispatcher();
</script>

<footer class="flex items-center p-4 justify-between">
  <span id="todo-count">
    <strong>{todosCount}</strong> items left
  </span>

  <ul class="flex gap-4">
    <li>
      <button
        class="link hover:text-blue-400"
        class:text-blue-400={status === 'all'}
        data-status="all"
        on:click={() => dispatch('status', 'all')}
        type="button"
      >
        All
      </button>
    </li>
    <li>
      <button
        class="link hover:text-blue-400"
        class:text-blue-400={status === 'pending'}
        data-status="pending"
        type="button"
        on:click={() => dispatch('status', 'pending')}
      >
        Active
      </button>
    </li>
    <li>
      <button
        class="link hover:text-blue-400"
        class:text-blue-400={status === 'done'}
        data-status="done"
        type="button"
        on:click={() => dispatch('status', 'done')}
      >
        Completed
      </button>
    </li>
  </ul>

  <div />
</footer>
