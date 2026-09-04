import {
  PanelsTopLeft,
  Tv,
} from "lucide-react";

import type { Widget } from "../types";

type WidgetListProps = {
  widgets: Widget[];

  selectedWidgetId: string;

  onSelect: (
    windowLabel: string
  ) => void;
};

export function WidgetList({
  widgets,
  selectedWidgetId,
  onSelect,
}: WidgetListProps) {
  return (
    <aside
      className="
        overflow-hidden
        rounded-3xl
        border
        border-white/[0.07]
        bg-[#121317]
      "
    >
      <div className="border-b border-white/[0.06] px-6 py-6">
        <div className="flex items-start justify-between">
          <div>
            <h2 className="font-semibold text-zinc-100">
              Installed widgets
            </h2>

            <p className="mt-1 text-sm text-zinc-500">
              {widgets.length} widgets installed
            </p>
          </div>

          <div
            className="
              flex
              h-9
              w-9
              items-center
              justify-center
              rounded-xl
              bg-[#1b1c21]
              text-zinc-500
            "
          >
            <PanelsTopLeft size={17} />
          </div>
        </div>
      </div>

      <div className="p-3">
        {widgets.map((widget) => {
          const selected =
            widget.windowLabel === selectedWidgetId;

          return (
            <button
              key={widget.windowLabel}
              type="button"
              onClick={() =>
                onSelect(widget.windowLabel)
              }
              className={`
                mb-1
                flex
                w-full
                items-center
                gap-3
                rounded-2xl
                px-3
                py-3
                text-left
                transition-colors

                ${
                  selected
                    ? `
                      bg-[#202126]
                      text-zinc-100
                    `
                    : `
                      text-zinc-400
                      hover:bg-[#191a1f]
                      hover:text-zinc-200
                    `
                }
              `}
            >
              <div
                className="
                  flex
                  h-10
                  w-10
                  shrink-0
                  items-center
                  justify-center
                  rounded-xl
                  bg-[#17181d]
                  text-zinc-400
                "
              >
                <Tv size={18} />
              </div>

              <div className="min-w-0 flex-1">
                <div className="truncate font-medium">
                  {widget.name}
                </div>

                <div className="mt-0.5 text-xs text-zinc-500">
                  {widget.id} | {widget.width}
                  ×
                  {widget.height}
                </div>
              </div>

              <div
                className={`
                  h-2
                  w-2
                  rounded-full

                  ${
                    widget.enabled
                      ? "bg-emerald-400"
                      : "bg-zinc-600"
                  }
                `}
              />
            </button>
          );
        })}
      </div>
    </aside>
  );
}