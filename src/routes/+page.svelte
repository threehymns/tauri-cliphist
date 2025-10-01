<script lang="ts">
import { createVirtualizer } from "@tanstack/svelte-virtual";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMount } from "svelte";

interface ClipboardEntry {
	id: string;
	content: string;
	content_type: string;
}

interface CliphistError {
	message: string;
}

let history = $state<ClipboardEntry[]>([]);
let searchQuery = $state("");
let error = $state<string | null>(null);

// Virtual list setup
let scrollElement = $state<HTMLElement>();

// Create virtualizer with reactive count
const virtualizer = $derived(
	createVirtualizer({
		count: history.length,
		getScrollElement: () => scrollElement ?? null,
		estimateSize: () => 80,
		overscan: 5,
		gap: 8,
	}),
);

async function loadHistory() {
	console.log("Loading history...");
	try {
		history = await invoke("get_history");
		console.log("History loaded, length:", history.length);
		error = null;
	} catch (e) {
		console.log("Failed to load history:", e);
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
		error = `Failed to copy content: ${(e as CliphistError).message}`;
		console.error("Failed to copy content:", e);
	}
}

async function deleteEntry(id: string) {
	console.log("Deleting entry with id:", id);
	try {
		await invoke("delete_entry", { id });
		console.log("Delete invoke succeeded");
		await search();
		console.log("History reloaded, new length:", history.length);
		error = null;
	} catch (e) {
		console.log("Delete failed:", e);
		error = (e as CliphistError).message;
		console.error("Failed to delete entry:", e);
	}
}

onMount(async () => {
	try {
		const available = await invoke("is_cliphist_available");
		if (!available) {
			error =
				"cliphist is not installed or not available. Please install cliphist to use this app.";
			return;
		}
	} catch (e) {
		error = `Failed to check cliphist availability: ${(e as CliphistError).message}`;
		return;
	}
	await loadHistory();

	// Add ESC key listener to close app
	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === "Escape") {
			e.preventDefault();
			getCurrentWindow().close();
		}
	};
	window.addEventListener("keydown", handleKeydown, { capture: true });
});
</script>

<main class="rounded-3xl dark:bg-neutral-950 bg-neutral-200 p-4 max-w-4xl mx-auto h-screen flex flex-col" id="theme-root">
  {#if error}
    <div
      class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4 dark:bg-red-900 dark:border-red-600 dark:text-red-200"
    >
      <strong>Error:</strong>
      {error}
    </div>
  {/if}

  <div class="mb-4 flex gap-2 flex-shrink-0">
    <input
      bind:value={searchQuery}
      oninput={search}
      class="border border-neutral-300 rounded-lg p-4 flex-1 bg-white text-black dark:bg-neutral-800 dark:border-neutral-600 dark:text-white focus:border-neutral-500 focus:outline-none"
      placeholder="Search clipboard history..."
    />
  </div>

    <div
      class={`flex-1 overflow-auto rounded-2xl ${history.length === 0 ? "hidden" : ""}`}
      bind:this={scrollElement}
    >
      <div
        style="height: {$virtualizer.getTotalSize()}px; width: 100%; position: relative;"
      >
        {#each $virtualizer.getVirtualItems() as virtualItem (virtualItem.key)}
          {@const entry = history[virtualItem.index]}
          <div
            class="group border border-neutral-100 first:rounded-t-2xl last:rounded-b-2xl rounded-lg p-4 flex justify-between items-center bg-white dark:bg-neutral-900 hover:bg-neutral-50 dark:border-neutral-800 dark:hover:bg-neutral-800"
            style="position: absolute; top: 0; left: 0; width: 100%; height: {virtualItem.size}px; transform: translateY({virtualItem.start}px);"
          >
            <div class="flex-1 min-w-0">
              <p
                class="text-sm whitespace-pre-wrap break-words text-black dark:text-white"
              >
                {entry.content}
              </p>
            </div>
            <div class="absolute end-2 group-hover:flex flex-col gap-2 ml-4 hidden font-medium">
              <button
                onclick={() => copyContent(entry)}
                class="rounded-full border border-green-500 hover:border-green-600 bg-green-200 hover:bg-green-300 text-green-800 dark:text-green-400 px-3 py-1 text-sm dark:border-green-800 dark:bg-green-950 dark:hover:bg-green-900"
              >
                Copy
              </button>
              <button
                onclick={() => deleteEntry(entry.id)}
                class="rounded-full border border-red-500 hover:border-red-600 bg-red-200 hover:bg-red-300 text-red-800 dark:text-red-400 px-3 py-1 text-sm dark:border-red-800 dark:bg-red-950 dark:hover:bg-red-900"
              >
                Delete
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    {#if history.length === 0}
    <p class="text-neutral-500 text-center py-8 dark:text-neutral-400">
      No clipboard history found. <br /> Make sure cliphist is installed and has stored
      clipboard content.
    </p>
  {/if}
</main>
