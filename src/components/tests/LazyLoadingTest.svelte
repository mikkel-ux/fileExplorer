<script lang="ts">
  let { children } = $props();
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

<div bind:this={rootElement} class="h-full w-full overflow-auto">
  {#if renderComponent}
    <div transition:fade>
      {@render children()}
    </div>
  {/if}
</div>
