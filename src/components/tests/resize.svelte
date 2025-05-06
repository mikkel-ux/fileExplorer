<script lang="ts">
  import { dndzone, dragHandle, dragHandleZone } from "svelte-dnd-action";

  type itemType = { id: number; name: string };

  let box1Ratio: number = $state(0.5);
  let isDragging: boolean = $state(false);
  let items: itemType[] = $state([
    { id: 1, name: "Item 1" },
    { id: 2, name: "Item 2" },
  ]);

  let items2: itemType[] = $state([]);

  $effect(() => {
    if (items.length === 1) {
      box1Ratio = 1;
    } else if (items.length === 2) {
      box1Ratio = 0.5;
    }
  });

  const startDragging = (e: MouseEvent) => {
    isDragging = true;
    document.body.style.cursor = "col-resize";
    window.addEventListener("mousemove", onDrag);
    window.addEventListener("mouseup", stopDragging);
  };

  const stopDragging = () => {
    isDragging = false;
    document.body.style.cursor = "";
    window.removeEventListener("mousemove", onDrag);
    window.removeEventListener("mouseup", stopDragging);
  };

  const onDrag = (e: MouseEvent) => {
    if (!isDragging) return;
    const container = document.getElementById("resize-container");
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const newRatio = (e.clientX - rect.left) / rect.width;
    box1Ratio = Math.min(0.9, Math.max(0.1, newRatio));
  };

  function handleFooSort(e: any) {
    items = e.detail.items;
  }
  function handleFoo2Sort(e: any) {
    items2 = e.detail.items;
  }
</script>

<section
  class="flex flex-row p-2 gap-5 border-2 border-gray-400 rounded-lg h-40 w-full max-w-xl"
  use:dndzone={{
    items: items2,
  }}
  onconsider={handleFoo2Sort}
  onfinalize={handleFoo2Sort}
>
  {#each items2 as item, i (item.id)}
    <div class="h-full border-2 border-gray-200 relative group w-full">
      <div class="flex items-center justify-center h-full bg-gray-200">
        <span class="text-black">{item.name}</span>
      </div>
    </div>
  {/each}
</section>
<!-- TODO fix the error that happens when trying to drag the view on the right of the button -->
<section
  id="resize-container"
  class="flex flex-row p-2 border-2 border-gray-400 rounded-lg h-40 w-full max-w-xl"
  use:dragHandleZone={{
    items: items,
    dragDisabled: isDragging,
  }}
  onconsider={handleFooSort}
  onfinalize={handleFooSort}
>
  {#each items as item, i (item.id)}
    <div
      class="h-full border-2 border-gray-200 relative group"
      style={`width: ${i === 0 ? box1Ratio * 100 : (1 - box1Ratio) * 100}%`}
    >
      <div class="flex items-center justify-center h-full bg-gray-200">
        <span class="text-black">{item.name}</span>
        <span
          use:dragHandle
          aria-label="drag-handle for {item.name}"
          class="ml-2 cursor-move text-black">::</span
        >
      </div>
    </div>

    {#if i === 0 && items.length > 1}
      <button
        type="button"
        class="right-0 top-0 h-full w-2 cursor-col-resize bg-[#ccc] hover:bg-[#999] appearance-none border-none p-0"
        onmousedown={startDragging}
        aria-label="Resize views"
        aria-describedby="resize-hint"
        tabindex="0"
        onkeydown={(e) => {
          if (e.key === "ArrowLeft")
            box1Ratio = Math.max(0.1, box1Ratio - 0.01);
          if (e.key === "ArrowRight")
            box1Ratio = Math.min(0.9, box1Ratio + 0.01);
        }}
      ></button>
    {/if}
  {/each}
</section>
