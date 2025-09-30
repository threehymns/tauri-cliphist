<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { createVirtualizer } from "@tanstack/svelte-virtual";

  interface ClipboardEntry {
    id: string;
    content: string;
    timestamp: string;
    content_type: string;
  }

  interface CliphistError {
    message: string;
  }

  let history = $state<ClipboardEntry[]>([]);
  let searchQuery = $state("");
  let error = $state<string | null>(null);

  function debounce(func: (...args: any[]) => void, wait: number) {
    let timeout: number;
    return function executedFunction(...args: any[]) {
      const later = () => {
        clearTimeout(timeout);
        func(...args);
      };
      clearTimeout(timeout);
      timeout = setTimeout(later, wait);
    };
  }

  const debouncedSearch = debounce(search, 300);

  // Virtual list setup
  let scrollElement = $state<HTMLElement>();

  // Create virtualizer with reactive count
  const virtualizer = $derived(createVirtualizer({
    count: history.length,
    getScrollElement: () => scrollElement ?? null,
    estimateSize: () => 120,
    overscan: 5,
  }));

  async function loadHistory() {
    try {
      history = await invoke("get_history");
      error = null;
    } catch (e) {
      error = (e as CliphistError).message;
      console.error("Failed to load history:", e);
    }
  }

  async function search() {
    if (searchQuery.trim()) {
      try {
        history = await invoke("search_history", { query: searchQuery });
        error = null;
      } catch (e) {
        error = (e as CliphistError).message;
        console.error("Failed to search history:", e);
      }
    } else {
      await loadHistory();
    }
  }

  async function copyContent(entry: ClipboardEntry) {
    try {
      // Get the full content from cliphist
      const fullContent = await invoke("get_entry_content", { id: entry.id });
      await invoke("copy_to_clipboard", { content: fullContent });
      error = null;
    } catch (e) {
      error = (e as CliphistError).message;
      console.error("Failed to copy content:", e);
    }
  }

  async function deleteEntry(id: string) {
    try {
      await invoke("delete_entry", { id });
      await loadHistory();
      error = null;
    } catch (e) {
      error = (e as CliphistError).message;
      console.error("Failed to delete entry:", e);
    }
  }



  function formatTimestamp(timestamp: string): string {
    try {
      const date = new Date(timestamp);
      return date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: 'numeric',
        minute: '2-digit',
        hour12: true
      });
    } catch (e) {
      return timestamp; // Fallback to original if parsing fails
    }
  }

  onMount(async () => {
    await loadHistory();
  });
</script>

<main class="p-4 max-w-4xl mx-auto h-screen flex flex-col">
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4 dark:bg-red-900 dark:border-red-600 dark:text-red-200">
      <strong>Error:</strong> {error}
    </div>
  {/if}

  <div class="mb-4 flex gap-2 flex-shrink-0">
    <input
      bind:value={searchQuery}
      oninput={debouncedSearch}
      class="border border-gray-300 rounded px-3 py-2 flex-1 bg-white text-black dark:bg-gray-800 dark:border-gray-600 dark:text-white"
      placeholder="Search clipboard history..."
    />
  </div>

  <div
    class="border border-gray-200 rounded flex-1 overflow-auto bg-white dark:bg-gray-800 dark:border-gray-700"
    bind:this={scrollElement}
  >
    <div
      style="height: {$virtualizer.getTotalSize()}px; width: 100%; position: relative;"
    >
      {#each $virtualizer.getVirtualItems() as virtualItem (virtualItem.key)}
        {@const entry = history[virtualItem.index]}
        <div
          class="border-b border-gray-100 last:border-b-0 p-3 flex justify-between items-center hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-700"
          style="position: absolute; top: 0; left: 0; width: 100%; height: {virtualItem.size}px; transform: translateY({virtualItem.start}px);"
        >
          <div class="flex-1 min-w-0">
            <p class="text-sm text-gray-600 dark:text-gray-400">{formatTimestamp(entry.timestamp)}</p>
            <p class="whitespace-pre-wrap break-words truncate text-black dark:text-white">{entry.content}</p>
          </div>
          <div class="flex gap-2 ml-4 flex-shrink-0">
            <button
              onclick={() => copyContent(entry)}
              class="bg-green-500 hover:bg-green-600 text-white px-3 py-1 rounded text-sm dark:bg-green-600 dark:hover:bg-green-700"
            >
              Copy
            </button>
            <button
              onclick={() => deleteEntry(entry.id)}
              class="bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded text-sm dark:bg-red-600 dark:hover:bg-red-700"
            >
              Delete
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>

  {#if history.length === 0}
    <p class="text-gray-500 text-center py-8 dark:text-gray-400">
      No clipboard history found. Make sure cliphist is installed and has stored clipboard content.
    </p>
  {/if}
</main>