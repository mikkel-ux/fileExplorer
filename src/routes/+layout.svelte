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

<div class="flex flex-col h-screen">
  <header class="bg-gray-800 text-white p-4 grid grid-cols-[auto_1fr] gap-4">
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
    </div>

    <Search />
  </header>

  <main class="flex-1">
    <div class="grid grid-cols-[auto_1fr] h-full">
      <div class="border-r-2 p-4 border-black">
        <p class="text-black">home</p>
      </div>
      {@render children()}
    </div>
  </main>

  <footer class="bg-gray-800 text-white p-4">
    <p>footer</p>
  </footer>
</div>
