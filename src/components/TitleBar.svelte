<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { dndzone } from "svelte-dnd-action";
  import { flip } from "svelte/animate";
  /*   import {tabs, nextTabId} from "../stores/tabsStore"; */

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

  let tabs: Tab[] = $state([
    { name: "aaaaaaaaaaaaaaa", id: 1, isActive: true },
  ]);

  /* let newTabId = $state(
    tabs.length > 0 ? Math.max(...tabs.map((tab) => tab.id)) + 1 : 1
  ); */

  const addNewTab = () => {
    const lastTab = tabs[tabs.length - 1];
    const newTabId = lastTab ? lastTab.id : 0;
    const newTab = {
      name: `Tab ${tabs.length + 1}`,
      id: newTabId + 1,
      isActive: false,
    };
    tabs.push(newTab);
    setTabToActive(newTab.id);
  };

  const removeTab = (tabId: number) => {
    tabs = tabs.filter((tab) => tab.id !== tabId);
    console.log("Tab removed:", tabId);
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.ctrlKey && event.key.toLocaleLowerCase() === "t") {
      addNewTab();
    }
    if (event.ctrlKey && event.key.toLocaleLowerCase() === "w") {
      /* const activeTab = tabs[tabs.length - 1];
      if (activeTab) {
        removeTab(activeTab.id);
      } */
      console.log("Ctrl + W pressed, but tab removal is disabled for now.");
    }
  };

  const setTabToActive = (tabId: number) => {
    tabs = tabs.map((tab) => ({
      ...tab,
      isActive: tab.id === tabId,
    }));
  };

  function handleSort(e: any) {
    tabs = e.detail.items;
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
        items: tabs,
        flipDurationMs: 150,
        type: "tabs",
      }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex gap-2 items-center overflow-x-auto tab-scrollbar"
    >
      {#each tabs as tab (tab.id)}
        <div
          animate:flip={{ duration: 150 }}
          class={`flex items-center p-2 rounded-t-lg h-7 w-32 ${tab.isActive ? "bg-highlight" : "bg-secondary-bg"}`}
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
          <button class="ml-5 z-10" onclick={() => removeTab(tab.id)}>X</button>
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
</style>
