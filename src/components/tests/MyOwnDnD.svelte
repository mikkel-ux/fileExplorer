<script>
  let item = { id: 1, name: "Draggable Item" };

  function handleDragStart(event) {
    console.log("Drag start:", item);
    event.dataTransfer.setData("text/plain", item.id);
    event.dataTransfer.effectAllowed = "move"; // Allow move operation
  }

  function handleDrop(event) {
    event.preventDefault();
    console.log("Drop event triggered");
    const itemId = event.dataTransfer.getData("text/plain");
    console.log("Dropped item ID:", itemId);
    // Handle the drop logic here if needed
  }

  function handleDragOver(event) {
    event.preventDefault();
    console.log("Drag over event triggered");
    event.dataTransfer.dropEffect = "move"; // Indicate move operation
  }
</script>

<div>
  <div
    class="box"
    role="group"
    aria-label="Draggable Box"
    on:drop={handleDrop}
    on:dragover={handleDragOver}
  >
    <h3>Drag and Drop Box</h3>
    <div class="item" draggable="true" on:dragstart={handleDragStart}>
      {item.name}
    </div>
  </div>
</div>

<style>
  .box {
    width: 200px;
    height: 200px;
    border: 1px solid #000;
    margin: 20px;
    padding: 10px;
  }

  .item {
    padding: 10px;
    margin: 5px;
    border: 1px solid #ccc;
    cursor: grab;
  }
</style>
