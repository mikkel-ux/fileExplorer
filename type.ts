export type TabType = {
  id: number;
  name: string;
  isActive: boolean;
};

export type tabType = {
  id: number;
  layout: "single" | "split";
  view: viewType[];
  splitRatio: number;
  activeViewId: string;
};

export type viewType = {
  id: string;
  title: string;
  image: string;
  history: string[];
  currentIndex: number;
};
