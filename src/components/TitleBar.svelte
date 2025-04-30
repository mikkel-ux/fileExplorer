<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { dndzone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  import { fly, fade } from "svelte/transition";
  import TabList from "./tabs/tabList.svelte";
  import {
    tabs,
    nextTabId,
    activeTabId,
    isDragging,
  } from "../stores/tabsStore";
  import { tick } from "svelte";

  const appWindow = getCurrentWindow();

  const minimize = () => {
    appWindow.minimize();
  };

  const toggleMaximize = () => {
    appWindow.toggleMaximize();
  };

  const close = () => {
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

  const removeTab = async (tabId: number) => {
    tabs.update((all) => all.filter((tab) => tab.id !== tabId));
    await tick();

    if ($tabs.length > 0) {
      const lastOpenTab = $tabs[$tabs.length - 1];
      setTabToActive(lastOpenTab.id);
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
  };

  function handleSort(e: any) {
    isDragging.set(e.type === "finalize" ? false : true);
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
        type: "tabs",
        flipDurationMs: 150,
      }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex gap-2 items-center overflow-x-auto tab-scrollbar overflow-y-hidden"
    >
      <TabList onTabClick={setTabToActive} onTabClose={removeTab} />
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
  .tab-scrollbar {
    transition: width 0.15s ease;
  }

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

  /* Changed 'grab' to 'pointer' for elements targeted by dndzone */
  .tab-scrollbar [data-dndzone-item]:hover {
    cursor: pointer !important;
  }

  /* Overrides inline style "cursor: grab" */
  .tab-scrollbar div[style*="cursor: grab"] {
    cursor: pointer !important;
  }

  .tab-scrollbar .dnd-action-item-dragging,
  .tab-scrollbar [data-dndzone-item][style*="cursor"] {
    cursor: grabbing !important;
  }
</style>
