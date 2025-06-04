<script lang="ts">
  let items = [...Array(100).keys()];

  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let renderComponent = $state<boolean>(false);
  let rootElement = $state<HTMLElement | null>(null);

  onMount(() => {
    if (rootElement) {
      createIntersectionObserver().observe(rootElement);
    }
  });

  const createIntersectionObserver = () => {
    return new IntersectionObserver((entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          renderComponent = true;
        }
      }
    });
  };
</script>

<ul class="list-disc list-inside overflow-auto h-full" bind:this={rootElement}>
  {#each items as item}
    {#if renderComponent}
      <li transition:fade>{item}</li>
    {/if}
  {/each}
</ul>
