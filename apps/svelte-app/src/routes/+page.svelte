<script lang="ts">
  import { onMount } from 'svelte';
  import List from '../lib/List.svelte';
  import {
    useTodoDoneMutation,
    useTodoUndoneMutation,
    useTodoRemoveMutation,
    useTodosQuery,
    useTodoCreateMutation,
    useTodosCountQuery
  } from '../lib/query';
  import Footer from '../lib/Footer.svelte';

  let status: string | undefined;

  onMount(() => {
    status = new URLSearchParams(window.location.search).get('status') || 'all';
  });

  $: todosResult = useTodosQuery(
    { status: status || '' },
    {
      enabled: !!status
    }
  );

  const todoDoneMutation = useTodoDoneMutation();
  const todoUndoneMutation = useTodoUndoneMutation();
  const todoRemoveMutation = useTodoRemoveMutation();

  const todoCreateMutation = useTodoCreateMutation();

  async function handleTodoCreate(event: SubmitEvent) {
    const form = event.target as HTMLFormElement;
    const description = form.description.value;

    await $todoCreateMutation.mutateAsync({ description });
    form.reset();
  }

  async function handleTodoDone(event: CustomEvent<string>) {
    await $todoDoneMutation.mutateAsync({ id: event.detail });
  }

  async function handleTodoUndone(event: CustomEvent<string>) {
    await $todoUndoneMutation.mutateAsync({ id: event.detail });
  }

  async function handleTodoRemove(event: CustomEvent<string>) {
    if (!confirm('Are you sure you want to delete this todo?')) return;

    await $todoRemoveMutation.mutateAsync({ id: event.detail });
  }

  function handleStatusChange(event: CustomEvent<string>) {
    window.history.replaceState(null, '', `?status=${event.detail}`);
    status = event.detail;
  }
</script>

<svelte:head>
  <title>Todo App (Svelte)</title>
</svelte:head>

<div class="grid grid-rows-layout h-screen flex-1">
  <div class="">
    <header class="px-4 py-8">
      <h1>
        <span
          class="text-transparent font-extrabold text-5xl bg-gradient-to-r bg-clip-text from-pink-500 to-blue-500"
        >
          Todo App
        </span>
      </h1>

      <div class="pt-8 flex justify-center items-center gap-4">
        <form class="flex-1" id="new-todo" on:submit|preventDefault={handleTodoCreate}>
          <input
            type="text"
            name="description"
            placeholder="What needs to be done?"
            class="input input-bordered w-full"
            required
            minlength={3}
          />
        </form>
        <button
          class="btn btn-square"
          type="button"
          aria-label="Refresh todos list"
          disabled={false}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width={1.5}
            stroke="currentColor"
            class="w-6 h-6"
            class:animate-spin={$todosResult.isLoading || $todosResult.isRefetching}
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
            />
          </svg>
        </button>
      </div>
    </header>
  </div>

  <div class="overflow-auto">
    <main class="px-4">
      <List
        todos={$todosResult.data?.data || []}
        on:done={handleTodoDone}
        on:undone={handleTodoUndone}
        on:remove={handleTodoRemove}
      />
    </main>
  </div>

  <div class="">
    <Footer
      todosCount={$todosResult?.data?.informations.total || 0}
      on:status={handleStatusChange}
      {status}
    />
  </div>
</div>
