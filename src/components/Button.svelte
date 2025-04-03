<script lang="ts">
  export let className = "";

  let buttonProps = {
    class: `px-4 py-2 rounded-lg font-medium transition ${className} $$restProps.class`,
  };
</script>

<button on:click on:mouseover on:mouseenter on:mouseleave {...buttonProps}>
  <slot />
</button>
