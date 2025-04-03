<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import {
    addToHistory,
    getCurrentPath,
    goBack,
    goForward,
    getHistory,
  } from "../stores/historyStore";

  import Button from "../components/Button.svelte";

  const navigateToPage = () => {
    goto("/expolorer");
    addToHistory("/expolorer");
  };
</script>

<main class="h-full flex flex-col justify-center items-center">
  <button
    class="bg-blue-500 text-white p-2 rounded-lg mt-4"
    onclick={() => navigateToPage()}
  >
    Go to other page
  </button>

  <Button
    className="bg-green-500 text-white hover:bg-green-600"
    on:click={() => console.log("primary button clicked")}
  >
    Primary Button
  </Button>

  <Button
    className="bg-red-500 text-white hover:bg-red-600"
    on:click={() => console.log("danger button clicked")}
  >
    Danger Button
  </Button>
</main>
