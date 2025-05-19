export type testTabType = {
  id: number;
  name: string;
  isActive: boolean;
};

export type TabType = {
  id: number;
  layout: "single" | "split";
  view: ViewType[];
  splitRatio: number;
  activeViewId: string;
};

export type ViewType = {
  id: string;
  title: string;
  image: string;
  history: string[];
  currentIndex: number;
};
