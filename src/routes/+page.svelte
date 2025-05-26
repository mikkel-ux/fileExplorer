<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { addToHistory } from "../stores/historyStore";
  import { getPath, setPath } from "../stores/pathStore";
  import TabsDrag from "../components/tests/TabsDrag.svelte";
  import Resize from "../components/tests/resize.svelte";
  import Test from "../components/tests/Test.svelte";
  import Sidebar from "../components/Sidebar.svelte";
  import View from "./View.svelte";
  import FileFoulderInfo from "../components/FileFoulderInfo.svelte";

  import type { FileDataType } from "../../type";

  import Button from "../components/Button.svelte";
  import ArrowButton from "../components/ArrowButton.svelte";

  import PropsTest from "../components/PropsTest.svelte";

  let value = $state(0);
  let seclectedFile = $state<FileDataType | null>(null);

  $effect(() => {
    console.log(seclectedFile);
  });

  const navigateToPage = () => {
    goto("/expolorer");
    addToHistory("/expolorer");
  };

  const closeInfo = () => {
    console.log("close info");

    seclectedFile = null;
  };

  /* onMount(async () => {
    
  }) */
</script>

<!-- <main class="w-full h-full flex flex-row min-h-0"> -->
<!-- <button
    class="bg-blue-500 p-2 rounded-lg mt-4"
    onclick={() => navigateToPage()}
  >
    Go to other page
  </button> -->

<!-- <TabsDrag /> -->
<!-- <Resize /> -->

<!-- <Test /> -->

<section class="w-full h-full flex flex-row min-h-0">
  <Sidebar />
  <View bind:seclectedFile />
  {#if seclectedFile}
    <FileFoulderInfo {closeInfo} />
  {/if}
</section>

<!-- </main> -->

<style>
</style>
