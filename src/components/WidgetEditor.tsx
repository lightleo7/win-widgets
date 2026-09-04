import {
  Monitor,
  Move,
  SlidersHorizontal,
  RefreshCw,
  Trash2,
  RotateCcw
} from "lucide-react";

import {
  Input,
  Switch,
  Label,
  Button
} from "@heroui/react";


import type {
  Widget
} from "../types";

import { SectionCard } from "./SectionCard";
import { CustomSettings } from "./CustomSettings";

type WidgetEditorProps = {
  widget: Widget;

  onUpdate: (
    patch: Partial<Widget>
  ) => void;

  onUpdateSetting: (
    key: string,
    value: unknown
  ) => void;

  onReset: (
    widgetId: string,
    settings: Widget["settingDefinitions"]
  ) => void;

  onReload: (
    widgetId: string
  ) => void;

  onDelete: (
    widgetId: string
  ) => void;
};

export function WidgetEditor({
  widget,
  onUpdate,
  onUpdateSetting,
  onReset,
  onReload,
  onDelete,
}: WidgetEditorProps) {
  console.log("widget:", widget.id);
  console.log("settings:", widget.settings);
  console.log("definitions:", widget.settingDefinitions);

  return (
    <section className="min-w-0">
      <div className="mb-7 flex items-start justify-between">
        <div>
          <div className="flex items-center gap-3">
            <h2 className="text-2xl font-semibold tracking-tight text-zinc-100">
              {widget.name}
            </h2>

            <span
              className="
          rounded-lg
          bg-[#191a1f]
          px-2
          py-1
          font-mono
          text-[11px]
          text-zinc-400
        "
            >
              v{widget.version}
            </span>

            <span
              className="
          rounded-lg
          bg-[#191a1f]
          px-2
          py-1
          font-mono
          text-[11px]
          text-zinc-400
        "
            >
              {widget.id}
            </span>
          </div>

          <p className="mt-1 text-sm leading-relaxed text-zinc-400">
            {widget.description}
          </p>

          <div className="mt-1 flex items-center gap-2 text-xs text-zinc-500">
            <span>Created by</span>
            <span className="font-medium text-zinc-300">
              {widget.author}
            </span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Button
            size="sm"
            variant="tertiary"
            onPress={() => onReset(widget.id,
              widget.settingDefinitions)}
          >
            <RotateCcw size={15} />
            Reset
          </Button>

          <Button
            size="sm"
            variant="tertiary"
            onPress={() => onReload(widget.id)}
          >
            <RefreshCw size={15} />
            Reload
          </Button>

          <Button
            size="sm"
            variant="tertiary"
            onPress={() => onDelete(widget.id)}
            className="text-zinc-400 hover:text-red-400"
          >
            <Trash2 size={15} />
            Delete
          </Button>

          <Switch
            isSelected={widget.enabled}
            onChange={(enabled) =>
              onUpdate({
                enabled,
              })
            }
            aria-label="Enable widget"
          >
            <Switch.Content>
              <Switch.Control>
                <Switch.Thumb />
              </Switch.Control>
            </Switch.Content>
          </Switch>
        </div>
      </div>

      <div className="space-y-5">
        <EditorCard
          icon={<Monitor size={18} />}
          title="Size"
          description="Widget dimensions in pixels"
        >
          <div className="grid grid-cols-2 gap-5">
            <NumberField
              label="Width"
              value={widget.width}
              onChange={(width) =>
                onUpdate({
                  width,
                })
              }
            />

            <NumberField
              label="Height"
              value={widget.height}
              onChange={(height) =>
                onUpdate({
                  height,
                })
              }
            />
          </div>
        </EditorCard>

        <EditorCard
          icon={<Move size={18} />}
          title="Position"
          description="Widget coordinates on the desktop"
        >
          <div className="grid grid-cols-2 gap-5">
            <NumberField
              label="X"
              value={widget.x}
              onChange={(x) =>
                onUpdate({
                  x,
                })
              }
            />

            <NumberField
              label="Y"
              value={widget.y}
              onChange={(y) =>
                onUpdate({
                  y,
                })
              }
            />
          </div>
        </EditorCard>

        <SectionCard>
          <div className="flex items-center justify-between gap-6 px-6 py-5">
            <div className="flex items-center gap-4">
              <div
                className="
                  flex
                  h-10
                  w-10
                  shrink-0
                  items-center
                  justify-center
                  rounded-xl
                  bg-[#1b1c21]
                  text-zinc-400
                "
              >
                <SlidersHorizontal size={18} />
              </div>

              <div>
                <h3 className="font-semibold text-zinc-100">
                  Interaction
                </h3>

                <p className="mt-1 text-sm text-zinc-500">
                  Allow mouse interaction with this widget
                </p>
              </div>
            </div>

            <Switch
              isSelected={widget.interactive}
              onChange={(interactive) =>
                onUpdate({
                  interactive,
                })
              }
              aria-label="Interactive widget"
            >
              <Switch.Content>
                <Switch.Control>
                  <Switch.Thumb />
                </Switch.Control>
              </Switch.Content>
            </Switch>
          </div>
        </SectionCard>

        {widget.settingDefinitions.length > 0 && (
          <EditorCard
            icon={<SlidersHorizontal size={18} />}
            title="Widget settings"
            description="Customize widget behavior and appearance"
          >
            <CustomSettings
              definitions={widget.settingDefinitions}
              values={widget.settings ?? {}}
              onChange={onUpdateSetting}
            />
          </EditorCard>
        )}
      </div>
    </section>
  );
}

type EditorCardProps = {
  icon: React.ReactNode;
  title: string;
  description: string;
  children: React.ReactNode;
};

function EditorCard({
  icon,
  title,
  description,
  children,
}: EditorCardProps) {
  return (
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
            {icon}
          </div>

          <div>
            <h3 className="font-semibold text-zinc-100">
              {title}
            </h3>

            <p className="mt-1 text-sm text-zinc-500">
              {description}
            </p>
          </div>
        </div>
      </div>

      <div className="px-6 py-6">
        {children}
      </div>
    </SectionCard>
  );
}

type NumberFieldProps = {
  label: string;
  value: number;

  description?: string;

  min?: number;
  max?: number;
  step?: number;

  onChange: (
    value: number,
  ) => void;
};

function NumberField({
  label,
  value,
  description,
  min,
  max,
  step,
  onChange,
}: NumberFieldProps) {
  const id = label
    .toLowerCase()
    .replace(/\s+/g, "-");

  return (
    <div className="flex flex-col gap-2">
      <Label
        htmlFor={id}
        className="text-xs font-medium text-zinc-400"
      >
        {label}
      </Label>

      <Input
        id={id}
        type="number"
        value={String(value)}
        min={min}
        max={max}
        step={step}
        onChange={(event) => {
          const nextValue =
            Number(event.target.value);

          if (
            Number.isNaN(nextValue)
          ) {
            return;
          }

          onChange(nextValue);
        }}
        className="
          h-10
          w-full
          rounded-xl
          border
          border-white/[0.08]
          bg-[#191a1f]
          px-3
          text-sm
          text-zinc-100
          shadow-none
          outline-none
          placeholder:text-zinc-600
          hover:border-white/[0.12]
          focus:border-blue-500/70
          focus:ring-2
          focus:ring-blue-500/10
        "
      />

      {description && (
        <p className="text-xs text-zinc-500">
          {description}
        </p>
      )}
    </div>
  );
}