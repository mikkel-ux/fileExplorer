import { writable } from "svelte/store";

type TabType = {
  id: number;
  name: string;
  isActive: boolean;
};

export const nextTabId = writable(2);
export const tabs = writable<TabType[]>([
  {
    id: 1,
    name: "Home",
    isActive: true,
  },
]);
