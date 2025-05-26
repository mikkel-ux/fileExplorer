<script lang="ts">
  const close = () => {
    console.log("close button clicked");
  };
</script>

<section
  class="w-5 h-full flex flex-col gap-4 p-4 bg-red-600 hover:bg-red-700 transition-colors"
>
  <button class="btn btn-primary" on:click={() => close()}> x </button>
</section>
