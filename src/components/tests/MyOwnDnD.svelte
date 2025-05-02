<!-- svelte-ignore a11y_no_static_element_interactions -->
<script lang="ts">
  import { flip } from "svelte/animate";

  let containers = $state([
    { id: "div1", items: ["item1", "item2", "item3"] },
    { id: "div2", items: [] },
  ]);

  let draggedItem: string | null = $state(null);
  let hoverIndex = $state<number | null>(null);
  let hoverContainer = $state<string | null>(null);

  function handleDragStart(item: string) {
    draggedItem = item;
  }

  function handleDragOver(e: DragEvent, containerId: string, index: number) {
    e.preventDefault();
    hoverContainer = containerId;

    // Calculate hover position (before or after the item)
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const middleY = rect.top + rect.height / 2;

    hoverIndex = e.clientY < middleY ? index : index + 1;
  }

  function handleDrop(containerId: string) {
    if (!draggedItem) return;

    const container = containers.find((c) => c.id === containerId);
    if (!container) return;

    // Remove from all containers first
    containers = containers.map((c) => ({
      ...c,
      items: c.items.filter((i) => i !== draggedItem),
    }));

    // Add to target container at the correct position
    containers = containers.map((c) => {
      if (c.id === containerId) {
        let newItems = [...c.items];

        // If hovering between items, insert at that position
        if (hoverIndex !== null) {
          newItems.splice(hoverIndex, 0, draggedItem!);
        }
        // Otherwise just append
        else {
          newItems.push(draggedItem!);
        }

        return { ...c, items: newItems };
      }
      return c;
    });

    // Reset drag state
    draggedItem = null;
    hoverIndex = null;
    hoverContainer = null;
  }

  function handleDragLeave() {
    hoverIndex = null;
  }
</script>

<section class="flex gap-10">
  {#each containers as container}
    <div
      id={container.id}
      class="w-40 min-h-40 bg-blue-600 flex flex-col gap-2 p-2"
      ondrop={() => handleDrop(container.id)}
      ondragover={(e) => e.preventDefault()}
      ondragleave={handleDragLeave}
    >
      {#each container.items as item, index (item)}
        <!-- Drop target before each item -->
        <div
          class={`h-4 transition-all ${hoverContainer === container.id && hoverIndex === index ? "bg-yellow-200" : "bg-transparent"}`}
          ondragover={(e) => handleDragOver(e, container.id, index)}
          ondrop={() => {
            handleDrop(container.id);
          }}
        />

        <div
          class={`w-full h-10 bg-pink-600 ${draggedItem === item ? "opacity-50" : ""}`}
          draggable="true"
          ondragstart={() => handleDragStart(item)}
          ondragend={() => {
            draggedItem = null;
            hoverIndex = null;
          }}
        >
          {item}
        </div>
      {/each}

      <!-- Drop target at the end of container -->
      <div
        class={`h-4 transition-all ${hoverContainer === container.id && hoverIndex === container.items.length ? "bg-yellow-200" : "bg-transparent"}`}
        ondragover={(e) =>
          handleDragOver(e, container.id, container.items.length)}
        ondrop={() => {
          handleDrop(container.id);
        }}
      />
    </div>
  {/each}
</section>
