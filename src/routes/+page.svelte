<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { addToHistory } from "../stores/historyStore";
  import { getPath, setPath } from "../stores/pathStore";
  import TabsDrag from "../components/tests/TabsDrag.svelte";
  import Resize from "../components/tests/resize.svelte";

  import Button from "../components/Button.svelte";
  import ArrowButton from "../components/ArrowButton.svelte";

  import PropsTest from "../components/PropsTest.svelte";

  let value = $state(0);

  const navigateToPage = () => {
    goto("/expolorer");
    addToHistory("/expolorer");
  };

  /* onMount(async () => {
    
  }) */
</script>

<main class="h-full flex flex-col justify-center items-center">
  <!-- <button
    class="bg-blue-500 p-2 rounded-lg mt-4"
    onclick={() => navigateToPage()}
  >
    Go to other page
  </button> -->

  <!-- <TabsDrag /> -->
  <Resize />
</main>
