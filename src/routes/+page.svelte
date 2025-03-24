<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type file = {
    name: string;
  };

  let query = $state("home/");
  let greetMsg = $state<file[]>([]);

  async function greet() {
    if (query.trim() === "") return;
    greetMsg = await invoke("search_files", { query });
  }

  $effect(() => {
    greet();
  });
</script>

<main class="h-screen flex flex-col justify-center items-center">
  <input
    id="greet-input"
    placeholder="Enter a name..."
    bind:value={query}
    class="border p-2 rounded-lg"
    autocomplete="off"
  />
  {#if greetMsg}
    {#each greetMsg as msg}
      <p>{msg.name}</p>
    {/each}
  {/if}
</main>
