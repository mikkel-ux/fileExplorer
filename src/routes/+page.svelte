<script lang="ts">
  import { goto } from "$app/navigation";
  import { addToHistory } from "../stores/historyStore";

  import Button from "../components/Button.svelte";
  import ArrowButton from "../components/ArrowButton.svelte";

  import PropsTest from "../components/PropsTest.svelte";

  let value = $state(0);

  $effect(() => {
    console.log("in pervent", value);
  });

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

  <p class="text-black">{value}</p>
  <PropsTest bind:value />
</main>
