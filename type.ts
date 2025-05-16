export type TabType = {
  id: number;
  name: string;
  isActive: boolean;
};

export type tabType = {
  id: number;
  isActive: boolean;
  layout: "single" | "split";
  view: viewType[];
  splitRatio: number;
};

export type viewType = {
  id: number;
  title: string;
  image: string;
  history: string[];
  currentIndex: number;
};
