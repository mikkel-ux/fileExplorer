<script lang="ts">
  import {
    addTab,
    removeTab,
    setActiveTab,
    activeTabId,
    activeViewPerTab,
    testTabsStore,
    addView,
    removeView,
    setActiveView,
    getHistory,
    updateHistory,
    goBack,
    goForward,
    getCurrentPath,
    canGoBack,
    canGoForward,
    activeTabIndex,
    activeViews,
  } from "../../stores/testTabsStore";
  import { onMount } from "svelte";

  onMount(() => {
    console.log("Tabs:", $testTabsStore);
  });

  const updateTheHistory = () => {
    updateHistory("/expolorer");
    const history = getHistory();
    console.log("History:", history);
  };

  const goBackHistory = () => {
    goBack();
    const currentPath = getCurrentPath();
    console.log("Current Path:", currentPath);
  };

  const goForwardHistory = () => {
    goForward();
    const currentPath = getCurrentPath();
    console.log("Current Path:", currentPath);
  };

  const addNewView = () => {
    const newView = {
      id: 0,
      title: "New View",
      image: "path/to/image.png",
      history: ["/"],
      currentIndex: 0,
    };
    addView($activeTabId, newView);
  };

  const removeExistingView = () => {
    const activeTab = $activeTabId;
    removeView($activeTabId, $activeViewPerTab[activeTab]);
  };

  const changeCurentView = (viewid: number) => {
    setActiveView(viewid);
    const history = getHistory();
    const currentPath = getCurrentPath();
    console.log("----------------");
    console.log(
      "view id",
      viewid,
      "History:",
      history,
      "Current Path:",
      currentPath
    );
    console.log("----------------");
  };

  const addNewTab = () => {
    addTab();
  };

  const removeExistingTab = (tabId: number) => {
    removeTab(tabId);
  };

  const activeTab = (tabId: number) => {
    setActiveTab(tabId);
    console.log(
      "Active Tab ID:",
      tabId,
      "Tabs:",
      $testTabsStore[$activeTabIndex].view
    );
  };
</script>

<div class="flex flex-row gap-2">
  <button class="bg-blue-500 p-2 rounded-lg mt-4">{$canGoBack} back</button>
  <button class="bg-blue-500 p-2 rounded-lg mt-4"
    >{$canGoForward} forword</button
  >
</div>

<div class="flex flex-row gap-5">
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={updateTheHistory}
    >updateHistory</button
  >
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={goBackHistory}
    >goBack</button
  >
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={goForwardHistory}
    >goForward</button
  >
</div>

<div class="flex flex-row gap-5">
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={addNewView}
    >addView</button
  >
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={removeExistingView}
    >removeView</button
  >
  <div class="flex flex-row gap-2">
    {#each $activeViews as view}
      <button
        class="bg-blue-500 p-2 rounded-lg mt-4"
        onclick={() => changeCurentView(view.id)}>view {view.id}</button
      >
    {/each}
  </div>
</div>

<div class="flex flex-col gap-2 h-60 overflow-y-auto pb-10">
  <button class="bg-blue-500 p-2 rounded-lg mt-4" onclick={addNewTab}
    >addNewTab</button
  >
  {#each $testTabsStore as tab}
    <div class="bg-gray-200 p-2 rounded-lg text-black">
      <p>{tab.id}</p>
      <p>{tab.isActive}</p>
      <button
        class="bg-blue-500 p-2 rounded-lg"
        onclick={() => activeTab(tab.id)}>active tab</button
      >
      <button
        class="bg-blue-500 p-2 rounded-lg"
        onclick={() => removeExistingTab(tab.id)}>remove tab</button
      >
    </div>
  {/each}
</div>
