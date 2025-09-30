<script>
  import '../app.css';
  import { themeManager } from '../lib/theme';
  import { onMount } from 'svelte';

  let { children } = $props();

  let resolvedTheme = $state('light');

  onMount(() => {
    resolvedTheme = themeManager.getResolvedTheme();
    const unsubscribe = themeManager.subscribe((theme) => {
      resolvedTheme = theme;
    });

    return unsubscribe;
  });
</script>

<main class="min-h-screen bg-white text-black dark:bg-gray-900 dark:text-white transition-colors">
  {@render children()}
</main>