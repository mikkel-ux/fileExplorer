<!-- svelte-ignore a11y-no-static-element-interactions -->
<script lang="ts">
  let containers = [
    { id: "div1", items: ["item1"] },
    { id: "div2", items: [] },
  ];

  let items = ["item1", "item2"];

  let draggedItem: string | null = null;

  function handleDragStart(item: string) {
    draggedItem = item;
  }

  function handleDrop(targetContainer: string) {
    if (draggedItem) {
      containers = containers
        .map((container) => {
          // Remove from all
          return {
            ...container,
            items: container.items.filter((i) => i !== draggedItem),
          };
        })
        .map((container) => {
          // Add to target
          if (container.id === targetContainer) {
            return {
              ...container,
              items: [...container.items, draggedItem!],
            };
          }
          return container;
        });

      draggedItem = null;
    }
  }
</script>

<section class="flex gap-10">
  {#each containers as container}
    <div
      id={container.id}
      class="w-20 h-20 bg-blue-600 flex justify-center items-center"
      ondrop={() => handleDrop(container.id)}
      ondragover={(e) => {
        e.preventDefault();
      }}
    >
      {#each container.items as item}
        <div
          class="w-10 h-10 bg-pink-600"
          draggable="true"
          ondragstart={() => handleDragStart(item)}
        ></div>
      {/each}
    </div>
  {/each}
</section>
