import { writable, get, derived } from "svelte/store";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { tabType, viewType } from "../../type";

const appWindow = getCurrentWindow();

const nextTabId = writable(2);

//=========================
// Window Management
//=========================

function close() {
  appWindow.close();
}

//=========================
// helper functions
//=========================

function createTab() {
  const newTab: tabType = {
    id: get(nextTabId),
    isActive: false,
    layout: "single",
    view: [
      {
        id: 1,
        title: `Tab ${get(nextTabId)}`,
        image: "https://",
        history: ["/"],
        currentIndex: 0,
      },
    ],
    splitRatio: 1,
  };
  nextTabId.update((n) => n + 1);
  return newTab;
}

function findIndex(tabs: tabType[], id: number): number {
  return tabs.findIndex((tab) => tab.id === id);
}

function findActiveView(
  tabs: tabType[],
  activeTabIdVal: number,
  viewMap: Record<number, number>
): viewType | undefined {
  /* const activeTab = tabs.find((tab) => tab.isActive);
  return activeTab?.view.find((v) => v.id === viewId); */
  /* const activeTabIdVal = get(activeTabId);
  const viewMap = get(activeViewPerTab); */
  const tab = tabs.find((tab) => tab.id === activeTabIdVal);
  const viewId = viewMap?.[activeTabIdVal];
  return tab?.view.find((v) => v.id === viewId);
}

//=========================
// exported stores
//=========================
export const activeTabId = writable<number>(1);
export const activeViewPerTab = writable<Record<number, number>>({
  1: 1,
});
export const testTabsStore = writable<tabType[]>([
  {
    id: 1,
    isActive: true,
    layout: "single",
    view: [
      {
        id: 1,
        title: "Tab 1",
        image: "image1.png",
        history: ["/"],
        currentIndex: 0,
      },
    ],
    splitRatio: 1,
  },
]);

//=========================
// Tab Management
//=========================

export const addTab = () => {
  testTabsStore.update((tabs) => {
    const newTab = createTab();
    return [...tabs, newTab];
  });
};

export const removeTab = (id: number) => {
  if (get(testTabsStore).length === 1) {
    close();
  }

  const currentTabs = get(testTabsStore);
  const tabIndex = currentTabs.findIndex((tab) => tab.id === id);
  if (tabIndex === -1) return;

  const newTabs = currentTabs.filter((tab) => tab.id !== id);
  testTabsStore.set(newTabs);

  if (newTabs.length === 0) return;
  const nextActive = newTabs[Math.max(0, tabIndex - 1)] ?? newTabs[0];
  if (nextActive) {
    setActiveTab(nextActive.id);
  }
};

export const setActiveTab = (id: number) => {
  testTabsStore.update((tabs) => {
    return tabs.map((tab) => ({
      ...tab,
      isActive: tab.id === id,
    }));
  });

  const tabs = get(testTabsStore);
  const tab = tabs.find((tab) => tab.id === id);
  const firstViewId = tab?.view[0].id;

  if (firstViewId != null) {
    activeViewPerTab.update((v) => ({ ...v, [id]: firstViewId }));
  }
};

//=========================
// View Management
//=========================

export const addView = (tabId: number, view: viewType) => {
  testTabsStore.update((tabs) => {
    const tabIndex = findIndex(tabs, tabId);
    const existingViewId = tabs[tabIndex].view[0]?.id;
    const viewId = existingViewId === 1 ? 2 : 1;
    view.id = viewId;
    tabs[tabIndex].view.push(view);
    tabs[tabIndex].splitRatio = 0.5;
    tabs[tabIndex].layout = "split";
    return tabs;
  });
};

export const removeView = (tabId: number, viewId: number) => {
  testTabsStore.update((tabs) => {
    const tabIndex = findIndex(tabs, tabId);
    const viewIndex = tabs[tabIndex].view.findIndex(
      (view) => view.id === viewId
    );
    tabs[tabIndex].view.splice(viewIndex, 1);
    tabs[tabIndex].splitRatio = 1;
    tabs[tabIndex].layout = "single";
    return tabs;
  });
};

export const setActiveView = (viewId: number) => {
  activeViewPerTab.update((prev) => {
    const activeTabIdVal = get(activeTabId);
    return { ...prev, [activeTabIdVal]: viewId };
  });
};

export const setViewTitle = (tabId: number, viewId: number, title: string) => {
  testTabsStore.update((tabs) => {
    const tabIndex = findIndex(tabs, tabId);
    const viewIndex = tabs[tabIndex].view.findIndex(
      (view) => view.id === viewId
    );
    tabs[tabIndex].view[viewIndex].title = title;
    return tabs;
  });
};

//=========================
// History Management
//=========================

export const updateHistory = (path: string) => {
  testTabsStore.update((tabs) => {
    const activeTabIdVal = get(activeTabId);
    const viewMap = get(activeViewPerTab);
    const activeView = findActiveView(tabs, activeTabIdVal, viewMap);
    if (activeView) {
      const newHistory = [
        ...activeView.history.slice(0, activeView.currentIndex + 1),
        path,
      ];
      activeView.history = newHistory;
      activeView.currentIndex = newHistory.length - 1;
    }
    return tabs;
  });
};

export const goBack = () => {
  testTabsStore.update((tabs) => {
    const activeTabIdVal = get(activeTabId);
    const viewMap = get(activeViewPerTab);
    const activeView = findActiveView(tabs, activeTabIdVal, viewMap);
    if (activeView) {
      const newIndex = Math.max(activeView.currentIndex - 1, 0);
      activeView.currentIndex = newIndex;
    }
    return tabs;
  });
};

export const goForward = () => {
  testTabsStore.update((tabs) => {
    const activeTabIdVal = get(activeTabId);
    const viewMap = get(activeViewPerTab);
    const activeView = findActiveView(tabs, activeTabIdVal, viewMap);
    if (activeView) {
      const newIndex = Math.min(
        activeView.currentIndex + 1,
        activeView.history.length - 1
      );
      activeView.currentIndex = newIndex;
    }
    return tabs;
  });
};

export const getCurrentPath = () => {
  let currentPath: string = "";
  const tabs = get(testTabsStore);
  const activeTabIdVal = get(activeTabId);
  const viewMap = get(activeViewPerTab);
  const activeView = findActiveView(tabs, activeTabIdVal, viewMap);
  if (activeView) {
    currentPath = activeView.history[activeView.currentIndex];
  }
  return currentPath;
};

export const getHistory = () => {
  let history: string[] = [];
  const tabs = get(testTabsStore);
  const activeTabIdVal = get(activeTabId);
  const viewMap = get(activeViewPerTab);
  const activeView = findActiveView(tabs, activeTabIdVal, viewMap);
  if (activeView) {
    history = activeView.history;
  }
  return history;
};

export const canGoBack = derived(
  [testTabsStore, activeViewPerTab, activeTabId],
  ([$historyStore, $activeViewPerTab, $activeTabId]) => {
    const tab = $historyStore.find((tab) => tab.id === $activeTabId);
    const viewId = $activeViewPerTab[$activeTabId];
    const activeView = tab?.view.find((v) => v.id === viewId);
    return activeView ? activeView.currentIndex > 0 : false;
  }
);

export const canGoForward = derived(
  [testTabsStore, activeViewPerTab, activeTabId],
  ([$historyStore, $activeViewPerTab, $activeTabId]) => {
    const tab = $historyStore.find((tab) => tab.id === $activeTabId);
    const viewId = $activeViewPerTab[$activeTabId];
    const activeView = tab?.view.find((v) => v.id === viewId);
    return activeView
      ? activeView.currentIndex < activeView.history.length - 1
      : false;
  }
);

export const activeTabIndex = derived(testTabsStore, ($testTabsStore) => {
  const activeTab = $testTabsStore.find((tab) => tab.isActive);
  return activeTab ? $testTabsStore.indexOf(activeTab) : -1;
});

export const activeViews = derived(
  [testTabsStore, activeTabIndex],
  ([$testTabsStore, $activeTabIndex]) => {
    if ($activeTabIndex === -1) return [];
    return $testTabsStore[$activeTabIndex].view;
  }
);
