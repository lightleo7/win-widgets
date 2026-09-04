import { Switch } from "@heroui/react";
import { Settings } from "lucide-react";

import { SectionCard } from "./SectionCard";

type SettingsPageProps = {
  startWithWindows: boolean;

  restoreWidgets: boolean;

  onStartWithWindowsChange: (
    value: boolean
  ) => void;

  onRestoreWidgetsChange: (
    value: boolean
  ) => void;
};



export function SettingsPage({
  startWithWindows,
  onStartWithWindowsChange,
}: SettingsPageProps) {
  return (
    <div className="max-w-[850px]">

      <SectionCard>

        <div className="border-b border-white/[0.06] px-6 py-5">

          <div className="flex items-center gap-4">

            <div
              className="
                flex
                h-10
                w-10
                items-center
                justify-center
                rounded-xl
                bg-[#1b1c21]
                text-zinc-400
              "
            >
              <Settings size={18} />
            </div>

            <div>

              <h2 className="font-semibold text-zinc-100">
                Application
              </h2>

              <p className="mt-1 text-sm text-zinc-500">
                General application behavior
              </p>

            </div>

          </div>

        </div>

        <div className="divide-y divide-white/[0.06]">

          <SettingRow
            title="Start with Windows"
            description="Automatically launch Win Widgets when Windows starts."
            value={startWithWindows}
            onChange={onStartWithWindowsChange}
          />

          {/* <SettingRow
            title="Restore widgets after restart"
            description="Recreate enabled widgets after launching the application."
            value={restoreWidgets}
            onChange={onRestoreWidgetsChange}
          /> */}

        </div>

      </SectionCard>

    </div>
  );
}


type SettingRowProps = {
  title: string;

  description: string;

  value: boolean;

  onChange: (
    value: boolean
  ) => void;
};

function SettingRow({
  title,
  description,
  value,
  onChange,
}: SettingRowProps) {
  return (
    <div className="flex items-center justify-between gap-8 px-6 py-5">

      <div>

        <h3 className="font-medium text-zinc-200">
          {title}
        </h3>

        <p className="mt-1 text-sm text-zinc-500">
          {description}
        </p>

      </div>

      <Switch
        isSelected={value}
        onChange={onChange}
        aria-label={title}
        className="group"
      >
        <Switch.Content>
          <Switch.Control>
            <Switch.Thumb />
          </Switch.Control>
        </Switch.Content>
      </Switch>

    </div>
  );
}