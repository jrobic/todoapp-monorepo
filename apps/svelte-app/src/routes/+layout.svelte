<script lang="ts">
  import '../styles/tailwind.css';
  import { QueryClientProvider } from '@sveltestack/svelte-query';
  import { queryClient } from '$lib/queryClient';
</script>

<QueryClientProvider client={queryClient}>
  <slot />
</QueryClientProvider>
