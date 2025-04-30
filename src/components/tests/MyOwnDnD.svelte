<script>
  let draggedItem = null;
  let dragOverIndex = null;
  let items = ["Item 1", "Item 2", "Item 3", "Item 4"];

  function handleDragStart(e, item) {
    e.dataTransfer.setData("text/plain", item);
    draggedItem = item;
  }

  function handleDragEnter(index) {
    dragOverIndex = index;
  }

  function handleDragLeave() {
    dragOverIndex = null;
  }

  function handleDragOver(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }

  function handleDrop(e, targetIndex) {
    e.preventDefault();
    const item = e.dataTransfer.getData("text/plain");
    if (!item) return;

    const draggedIndex = items.indexOf(item);
    if (draggedIndex === -1) return;

    // Remove from old position and insert at new position
    items = items.filter((i) => i !== item);

    // Adjust target index if dragging downward
    const adjustedIndex =
      draggedIndex < targetIndex ? targetIndex - 1 : targetIndex;

    items = [
      ...items.slice(0, adjustedIndex),
      item,
      ...items.slice(adjustedIndex),
    ];

    draggedItem = null;
    dragOverIndex = null;
  }

  function handleKeyDown(e, index) {
    if (e.key === " " || e.key === "Enter") {
      e.preventDefault();
      // Implement keyboard drag initiation here
    } else if (e.key === "ArrowUp" && index > 0) {
      e.preventDefault();
      // Move item up
      [items[index], items[index - 1]] = [items[index - 1], items[index]];
      items = [...items];
    } else if (e.key === "ArrowDown" && index < items.length - 1) {
      e.preventDefault();
      // Move item down
      [items[index], items[index + 1]] = [items[index + 1], items[index]];
      items = [...items];
    }
  }
</script>

<div class="drag-container" role="list">
  {#each items as item, index (item)}
    <button
      draggable="true"
      on:dragstart={(e) => handleDragStart(e, item)}
      on:dragenter={() => handleDragEnter(index)}
      on:dragleave={handleDragLeave}
      on:dragover={handleDragOver}
      on:drop={(e) => handleDrop(e, index)}
      on:keydown={(e) => handleKeyDown(e, index)}
      aria-grabbed={draggedItem === item}
      class="draggable-item"
      class:drop-above={dragOverIndex === index &&
        items.indexOf(draggedItem) < index}
      class:drop-below={dragOverIndex === index &&
        items.indexOf(draggedItem) > index}
    >
      {item}
      <span class="sr-only"
        >Press space or enter to grab, arrow keys to move, space or enter to
        drop</span
      >
    </button>
  {/each}
</div>

<style>
  .drag-container {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px;
    list-style: none;
    margin: 0;
  }

  .draggable-item {
    padding: 12px;
    background: #f8f8f8;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    cursor: grab;
    transition: all 0.2s ease;
    text-align: left;
    width: 100%;
  }

  .draggable-item:active {
    cursor: grabbing;
  }

  .draggable-item[aria-grabbed="true"] {
    opacity: 0.5;
  }

  .drop-above {
    border-top: 2px solid #4caf50;
    margin-bottom: 12px;
  }

  .drop-below {
    border-bottom: 2px solid #4caf50;
    margin-top: 12px;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }
</style>
