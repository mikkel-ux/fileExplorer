<script lang="ts">
  import { fade, fly } from "svelte/transition";
  let { closeInfo } = $props();
  const close = () => {
    closeInfo();
  };
</script>

<section
  in:fly={{ x: 100, duration: 300 }}
  out:fade={{ duration: 300 }}
  class="w-2xl h-full flex flex-col gap-4 p-4 bg-red-600 transition-colors"
>
  <button
    class="btn btn-primary bg-blue-400 hover:bg-blue-900"
    onclick={() => close()}
  >
    x
  </button>
</section>
