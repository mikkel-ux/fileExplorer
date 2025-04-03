<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type file = {
    name: string;
  };

  let query = $state("home/");
  let files = $state<file[]>([]);
  let showFiles = $state(false);

  async function greet() {
    if (query.trim() === "") return;
    files = await invoke("search_files", { query });
  }

  $effect(() => {
    greet();
  });
</script>

<div class="relative w-full">
  <input
    placeholder="Enter a name..."
    bind:value={query}
    class="border p-2 rounded-lg bg-gray-100 text-black w-full"
    autocomplete="off"
    onfocus={() => (showFiles = true)}
    onblur={() => setTimeout(() => (showFiles = false), 50)}
  />
  {#if showFiles && files.length}
    <div
      class="absolute left-0 right-0 bg-gray-100 max-h-30 overflow-y-auto border rounded-lg shadow-lg z-10"
    >
      {#each files as file}
        <div
          class="p-2 border-b border-gray-300 hover:bg-gray-200 cursor-pointer text-black"
        >
          <p>{file.name}</p>
        </div>
      {/each}
    </div>
  {/if}
</div>
