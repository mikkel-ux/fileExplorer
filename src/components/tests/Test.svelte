<script lang="ts">
  import { dndzone } from "svelte-dnd-action";
  type itemType = { id: number; name: string; isActive: boolean };

  let items: itemType[] = [];

  function handleFoo2Sort(e: any) {
    items = e.detail.items;
  }
</script>

<section
  class="flex flex-row p-2 gap-5 border-2 border-gray-400 rounded-lg h-40 w-full max-w-xl"
  use:dndzone={{
    items: items,
    centreDraggedOnCursor: true,
    type: "foo",
  }}
  onconsider={handleFoo2Sort}
  onfinalize={handleFoo2Sort}
>
  {#each items as item (item.id)}
    <div class="h-full border-2 border-gray-200 relative group w-full">
      <div class="flex items-center justify-center h-full bg-gray-200">
        <span class="text-black">{item.name}</span>
      </div>
    </div>
  {/each}
</section>
