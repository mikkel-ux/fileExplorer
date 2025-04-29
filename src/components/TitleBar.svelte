<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { dndzone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import { tabs, nextTabId, activeTabId } from "../stores/tabsStore";

  type Tab = {
    name: string;
    id: number;
    isActive?: boolean;
  };

  const appWindow = getCurrentWindow();

  const minimize = () => {
    console.log("Minimizing window");

    appWindow.minimize();
  };

  const toggleMaximize = () => {
    console.log("Toggling window maximize state");
    appWindow.toggleMaximize();
  };

  const close = () => {
    console.log("Closing window");
    appWindow.close();
  };

  const addNewTab = () => {
    let id: number = 0;
    nextTabId.update((n) => {
      id = n;
      return n + 1;
    });

    const newTab = {
      name: `Tab ${$tabs.length + 1}`,
      id,
      isActive: false,
    };

    tabs.update((all) => [...all, newTab]);
    setTabToActive(id);
    console.log("New tab added:", newTab);
  };

  const removeTab = (tabId: number) => {
    tabs.update((all) => all.filter((tab) => tab.id !== tabId));
    console.log("Tab removed:", tabId);
    if ($tabs.length > 0) {
      setTabToActive($tabs[0].id);
    }
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.ctrlKey && event.key.toLocaleLowerCase() === "t") {
      addNewTab();
    }
    if (event.ctrlKey && event.key.toLocaleLowerCase() === "w") {
      const activeTab = $tabs.find((tab) => tab.id === $activeTabId);
      if (activeTab) {
        removeTab(activeTab.id);
        console.log(activeTab, $activeTabId);
      }
      console.log("Ctrl + W pressed, but tab removal is disabled for now.");
    }
  };

  const setTabToActive = (tabId: number) => {
    tabs.update((all) => {
      return all.map((tab) => ({
        ...tab,
        isActive: tab.id === tabId,
      }));
    });
    activeTabId.set(tabId);

    console.log("Active tab set to:", tabId, $activeTabId);
  };

  function handleSort(e: any) {
    tabs.update(() => e.detail.items);
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<section data-tauri-drag-region class="flex pl-5 pr-1 gap-5">
  <div
    class="flex-1 gap-2 flex items-end overflow-hidden pt-3"
    data-tauri-drag-region
  >
    <div
      onwheel={(e) => {
        e.preventDefault();
        const scrollAmount = e.deltaY > 0 ? 100 : -100;
        e.currentTarget.scrollBy({ left: scrollAmount, behavior: "smooth" });
      }}
      use:dndzone={{
        items: $tabs,
        flipDurationMs: 150,
        type: "tabs",
      }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex gap-2 items-center overflow-x-auto tab-scrollbar pr-2 pl-2"
    >
      {#each $tabs as tab (tab.id)}
        <div
          animate:flip={{ duration: 150 }}
          class={`flex items-center p-2 rounded-t-lg h-7 w-32 ${tab.isActive ? "bg-highlight" : "bg-secondary-bg"} cursor-pointer`}
          role="button"
          tabindex="0"
          onclick={() => setTabToActive(tab.id)}
          onkeydown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setTabToActive(tab.id);
            }
          }}
        >
          <span class="overflow-hidden text-ellipsis whitespace-nowrap flex-1"
            >{tab.name}</span
          >
          <button
            class="ml-5 z-10"
            onclick={(e) => {
              e.stopPropagation(), removeTab(tab.id);
            }}>X</button
          >
        </div>
      {/each}
    </div>
    <button
      class="bg-secondary-bg text-white p-2 rounded-t-lg h-7 w-7 flex items-center justify-center justify-self-center"
      onclick={addNewTab}>+</button
    >
  </div>

  <div class="flex gap-2 items-center">
    <button onclick={minimize} class="hover:bg-gray-200">_</button>
    <button onclick={toggleMaximize} class="hover:bg-gray-200">[]</button>
    <button onclick={close} class="hover:bg-gray-200">X</button>
  </div>
</section>

<style>
  .tab-scrollbar::-webkit-scrollbar {
    height: 5px;
  }

  .tab-scrollbar::-webkit-scrollbar-track {
    background: #fff;
    border-radius: 10px;
  }

  .tab-scrollbar::-webkit-scrollbar-thumb {
    background: #888;
    border-radius: 10px;
  }

  .tab-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #555;
  }

  .tab-scrollbar * {
    cursor: default !important;
  }

  /* svelte-ignore unused-selector */
  .tab-scrollbar [data-dndzone-item]:hover {
    cursor: grab !important;
  }

  /* svelte-ignore unused-selector */
  .tab-scrollbar .dnd-action-item-dragging,
  .tab-scrollbar [data-dndzone-item][style*="cursor"] {
    cursor: grabbing !important;
  }
</style>
