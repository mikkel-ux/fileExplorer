<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { dndzone } from "svelte-dnd-action";
  import TabList from "./tabs/tabList.svelte";
  import {
    tabs,
    nextTabId,
    activeTabId,
    isDragging,
  } from "../stores/tabsStore";
  import { tick } from "svelte";
  import { get } from "svelte/store";

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
  };

  const removeTab = async (tabId: number) => {
    /* tabs.update((all) => all.filter((tab) => tab.id !== tabId));
    await tick(); */

    /* if ($tabs.length > 0) {
      const lastOpenTab = $tabs[$tabs.length - 1];
      setTabToActive(lastOpenTab.id);
    } */

    /* if ($tabs.length > 0) {
      const checkForLeftTabs = $tabs.find((tab) => tab.id !== tabId);
      if (checkForLeftTabs) {
        setTabToActive(checkForLeftTabs.id);
      }
    } */

    if ($tabs.length === 1) {
      close();
    }

    const currentTabs = get(tabs);
    const tabIndex = currentTabs.findIndex((tab) => tab.id === tabId);

    if (tabIndex === -1) return;

    const newTabs = currentTabs.filter((tab) => tab.id !== tabId);
    tabs.set(newTabs);
    await tick();

    if (newTabs.length === 0) return;

    const leftTabBar = newTabs[tabIndex - 1];

    const rightTabBar = newTabs[tabIndex] ?? newTabs[newTabs.length - 1];

    const nextActive = leftTabBar ?? rightTabBar;
    if (nextActive) {
      setTabToActive(nextActive.id);
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
    console.log("items", $tabs);
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
        dropTargetStyle: {},
      }}
      onconsider={handleSort}
      onfinalize={handleSort}
      class="flex gap-2 items-center overflow-x-auto tab-scrollbar overflow-y-hidden min-w-2 min-h-7"
    >
      <TabList onTabClick={setTabToActive} onTabClose={removeTab} />
    </div>
    <button
      class="bg-secondary-bg text-white p-2 rounded-t-lg h-7 w-7 flex items-center justify-center justify-self-center
      hover:bg-highlight hover:opacity-90 transition-all ease-in-out"
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
</style>
