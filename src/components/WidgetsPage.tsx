import type { Widget } from "../types";

import { WidgetList } from "./WidgetList";
import { WidgetEditor } from "./WidgetEditor";

type WidgetsPageProps = {
  widgets: Widget[];

  selectedWidget: Widget;

  onSelectWidget: (
    windowLabel: string
  ) => void;

  onUpdateWidget: (
    windowLabel: string,
    patch: Partial<Widget>
  ) => void;

  onUpdateSetting: (
    windowLabel: string,
    key: string,
    value: unknown
  ) => void;

  onResetWidget: (
    widgetId: string,
    settings: Widget["settingDefinitions"]
  ) => void;

  onReloadWidget: (
    widgetId: string
  ) => void;

  onDeleteWidget: (
    widgetId: string
  ) => void;
};

export function WidgetsPage({
  widgets,
  selectedWidget,
  onSelectWidget,
  onUpdateWidget,
  onUpdateSetting,
  onResetWidget,
  onReloadWidget,
  onDeleteWidget,
}: WidgetsPageProps) {
  return (
    <div className="grid min-h-[650px] grid-cols-[320px_1fr] gap-8">
      <WidgetList
        widgets={widgets}
        selectedWidgetId={
          selectedWidget.windowLabel
        }
        onSelect={onSelectWidget}
      />

      <WidgetEditor
        widget={selectedWidget}
        onUpdate={(patch) =>
          onUpdateWidget(
            selectedWidget.windowLabel,
            patch
          )
        }
        onUpdateSetting={(key, value) =>
          onUpdateSetting(
            selectedWidget.windowLabel,
            key,
            value
          )
        }
        onReset={onResetWidget}
        onReload={onReloadWidget}
        onDelete={onDeleteWidget}
        
      />
    </div>
  );
}