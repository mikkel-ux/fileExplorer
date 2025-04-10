<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  type file = {
    name: string;
  };

  let files = $state<file[]>([]);
  let showFiles = $state<boolean>(false);
  let path = $state<string>("home/");

  async function searchFile() {
    if (path.trim() === "") return;
    files = await invoke("search_files", { path });
  }

  onMount(async () => {
    path = await invoke("get_path", { path: "home" });
  });

  $effect(() => {
    searchFile();
  });

  const test = async () => {
    let test: String[] = [];
    try {
      test = await invoke("get_foulders", { path });
    } catch (error) {
      console.log("error", error);
    }
    const prettyPath = test.map((item) => {
      return item.replace(/\\/g, "/");
    });
    console.log("test", prettyPath);
  };
</script>

<div class="relative w-full">
  <input
    placeholder="Enter a name..."
    bind:value={path}
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
