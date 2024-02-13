import { browser } from '$app/environment';
import { QueryClient } from '@sveltestack/svelte-query';

console.log('browser', browser);
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      // staleTime: Infinity,
      enabled: browser
    }
  }
});
