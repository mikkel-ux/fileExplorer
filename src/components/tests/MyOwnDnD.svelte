<script lang="ts">
  let draggedEl: HTMLElement | null = null;

  function handlePointerDown(event: PointerEvent) {
    const target = event.currentTarget as HTMLElement;
    draggedEl = target;
    target.setPointerCapture(event.pointerId);
    target.style.position = "absolute";
    target.style.zIndex = "1000";
  }

  function handlePointerMove(event: PointerEvent) {
    if (!draggedEl) return;
    draggedEl.style.left = `${event.clientX - draggedEl.offsetWidth / 2}px`;
    draggedEl.style.top = `${event.clientY - draggedEl.offsetHeight / 2}px`;
  }

  function handlePointerUp(event: PointerEvent) {
    if (draggedEl) {
      draggedEl.style.position = "";
      draggedEl.style.left = "";
      draggedEl.style.top = "";
      draggedEl.style.zIndex = "";
      draggedEl.releasePointerCapture(event.pointerId);
      draggedEl = null;
    }
  }
</script>

<div
  on:pointerdown={handlePointerDown}
  on:pointermove={handlePointerMove}
  on:pointerup={handlePointerUp}
  class="bg-blue-500 p-4 text-white w-fit rounded cursor-pointer select-none"
>
  Drag me manually
</div>
