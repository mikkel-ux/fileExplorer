<script lang="ts">
  let box1Ratio: number = $state(0.5);
  let isDragging: boolean = $state(false);
  let items = $state([{ id: 1, name: "Item 1" }]);

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
</script>

<div
  class="flex flex-row p-2 border-2 border-gray-400 rounded-lg h-20 w-full max-w-xl"
  id="resize-container"
>
  <div
    class="h-full border-2 border-gray-200"
    style="width: {box1Ratio * 100}%"
  >
    <div class="flex items-center justify-center h-full bg-gray-200">
      <span class="text-black">Box 1</span>
    </div>
  </div>
  <button
    type="button"
    class="h-full w-2 cursor-col-resize bg-[#ccc] hover:bg-[#999] appearance-none border-none p-0"
    onmousedown={startDragging}
    aria-label="Resize box"
    tabindex="0"
    onkeydown={(e) => {
      if (e.key === "ArrowLeft") box1Ratio = Math.max(0.1, box1Ratio - 0.01);
      if (e.key === "ArrowRight") box1Ratio = Math.min(0.9, box1Ratio + 0.01);
    }}
  ></button>
  <div
    class="h-full border-2 border-gray-200"
    style="width: {(1 - box1Ratio) * 100}%"
  >
    <div class="flex items-center justify-center h-full bg-gray-200">
      <span class="text-black">Box 2</span>
    </div>
  </div>
</div>
