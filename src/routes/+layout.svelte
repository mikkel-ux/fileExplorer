<script>
  let { children } = $props();

  import { goto } from "$app/navigation";
  import {
    goBack,
    goForward,
    canGoBack,
    canGoForward,
    getHistory,
  } from "../stores/historyStore";
  import "../app.css";
  import Search from "../components/search.svelte";
  import ArrowButton from "../components/ArrowButton.svelte";
  import TitleBar from "../components/TitleBar.svelte";

  let canGoBackValue = $state(false);
  let canGoForwardValue = $state(false);

  $effect(() => {
    canGoBackValue = $canGoBack;
    canGoForwardValue = $canGoForward;
  });

  const navigateBack = () => {
    if (canGoBackValue) {
      const path = goBack();
      goto(path);
    }
  };

  const navigateForward = () => {
    if (canGoForwardValue) {
      const path = goForward();
      goto(path);
    }
  };

  /* const test = () => {
    const history = getHistory();
    console.log("history", history);
  }; */
</script>

<div class="h-screen w-full grid grid-cols-[10%_1fr] grid-rows-[10%_15%_1fr]">
  <TitleBar />
  <header
    class="col-start-2 bg-gray-800 text-white p-4 grid grid-cols-[auto_1fr] gap-4"
  >
    <section class="flex gap-2">
      <ArrowButton
        direction="left"
        disabled={!canGoBackValue}
        on:click={() => navigateBack()}
      ></ArrowButton>

      <ArrowButton
        direction="right"
        disabled={!canGoForwardValue}
        on:click={() => navigateForward()}
      ></ArrowButton>
    </section>
    <!-- <Search /> -->
  </header>
  <section
    class="border-r-2 p-4 border-black bg-secondary-bg col-start-1 row-start-2 row-end-4"
  >
    <p>home</p>
  </section>

  {@render children()}

  <!-- <header class="bg-gray-800 text-white p-4 grid grid-cols-[auto_1fr] gap-4">
    <div class="flex gap-2">
      <ArrowButton
        direction="left"
        disabled={!canGoBackValue}
        on:click={() => navigateBack()}
      ></ArrowButton>

      <ArrowButton
        direction="right"
        disabled={!canGoForwardValue}
        on:click={() => navigateForward()}
      ></ArrowButton>
    </div> -->

  <!-- <Search /> -->
  <!-- </header> -->
</div>

<!-- <main class="col-span-2">
    {@render children()}
  </main> -->
