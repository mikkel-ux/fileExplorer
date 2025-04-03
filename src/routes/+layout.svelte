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

  const test = () => {
    const history = getHistory();
    console.log("history", history);
  };
</script>

<div class="flex flex-col h-screen">
  <header class="bg-gray-800 text-white p-4 grid grid-cols-[auto_1fr] gap-4">
    <div class="flex gap-2">
      <button
        onclick={() => navigateBack()}
        class="bg-blue-500 text-white p-2 rounded-lg"
        class:opacity-50={!canGoBackValue}
        disabled={!canGoBackValue}
      >
        go back
      </button>

      <button
        onclick={() => navigateForward()}
        class="bg-blue-500 text-white p-2 rounded-lg"
        class:opacity-50={!canGoForwardValue}
        disabled={!canGoForwardValue}
      >
        go forward
      </button>
      <button onclick={() => test()}> see </button>
    </div>

    <Search />
  </header>

  <main class="flex-1">
    {@render children()}
  </main>

  <footer class="bg-gray-800 text-white p-4">
    <p>footer</p>
  </footer>
</div>
