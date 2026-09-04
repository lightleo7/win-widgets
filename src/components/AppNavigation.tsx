import {
  Tabs,
} from "@heroui/react";

import {
  Compass,
  Settings,
  Tv,
} from "lucide-react";

import type { TabKey } from "../types";

type AppNavigationProps = {
  activeTab: TabKey;

  onChange: (
    tab: TabKey
  ) => void;
};

const tabs = [
  {
    id: "widgets" as const,
    label: "Widgets",
    icon: Tv,
  },

  {
    id: "settings" as const,
    label: "Settings",
    icon: Settings,
  },

  {
    id: "discover" as const,
    label: "Discover",
    icon: Compass,
  },
];

const listClassName = [
  "rounded-xl",
  "border border-white/[0.08]",
  "bg-white/[0.03]",
  "p-1",

  // сами вкладки
  "**:data-[slot=tabs-tab]:rounded-lg",
  "**:data-[slot=tabs-tab]:bg-transparent",
  "**:data-[slot=tabs-tab]:text-zinc-500",
  "**:data-[slot=tabs-tab]:opacity-100",
  "**:data-[slot=tabs-tab]:transition-colors",

  // hover
  "**:data-[slot=tabs-tab]:data-[hovered=true]:not-data-[selected=true]:bg-white/[0.05]",
  "**:data-[slot=tabs-tab]:data-[hovered=true]:not-data-[selected=true]:text-zinc-300",

  // pressed
  "**:data-[slot=tabs-tab]:data-[pressed=true]:not-data-[selected=true]:bg-white/[0.08]",

  // focus
  "**:data-[slot=tabs-tab]:data-[focus-visible=true]:ring-2",
  "**:data-[slot=tabs-tab]:data-[focus-visible=true]:ring-blue-500/30",

  // выбранная вкладка
  "**:data-[slot=tabs-tab]:data-[selected=true]:font-medium",
  "**:data-[slot=tabs-tab]:data-[selected=true]:text-zinc-100",
  "**:data-[slot=tabs-tab]:shadow-none",

  // indicator
  "**:data-[slot=tabs-indicator]:rounded-lg",
  "**:data-[slot=tabs-indicator]:bg-[#202126]",
  "**:data-[slot=tabs-indicator]:shadow-[inset_0_1px_0_rgba(255,255,255,0.05)]",
].join(" ");

export function AppNavigation({
  activeTab,
  onChange,
}: AppNavigationProps) {
  return (
    <nav>
      <Tabs
        selectedKey={activeTab}
        onSelectionChange={(key) =>
          onChange(key as TabKey)
        }
      >
        <Tabs.ListContainer
          className="
            rounded-none
            bg-transparent
          "
        >
          <Tabs.List
            aria-label="Application navigation"
            className={listClassName}
          >
            {tabs.map((tab) => {
              const Icon = tab.icon;

              return (
                <Tabs.Tab
                  key={tab.id}
                  id={tab.id}
                  className="
                    flex
                    h-10
                    items-center
                    gap-2
                    px-4
                    text-sm
                  "
                >
                  <Icon
                    size={17}
                    strokeWidth={1.8}
                  />

                  {tab.label}

                  <Tabs.Indicator />
                </Tabs.Tab>
              );
            })}
          </Tabs.List>
        </Tabs.ListContainer>
      </Tabs>
    </nav>
  );
}