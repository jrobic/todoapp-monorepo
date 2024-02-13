<script lang="ts">
  import Item from './Item.svelte';
  import type { Todo } from './types/todo';

  export let todos: Todo[] = [];
</script>

<ul id="list-todos">
  {#each todos as todo}
    <li>
      <Item {todo} on:done on:undone on:remove />
    </li>
  {/each}
</ul>
