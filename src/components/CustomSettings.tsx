import {
    Input,
    Select,
    Switch,
    Slider,
    Label,
    ListBox,
    ColorArea,
    ColorField,
    ColorPicker,
    ColorSlider,
    ColorSwatch,
    ColorSwatchPicker,
    parseColor,
} from "@heroui/react";

import type {
    WidgetSettingDefinition,
    WidgetSettings,
} from "../types";

type CustomSettingsProps = {
    definitions: WidgetSettingDefinition[];
    values: WidgetSettings;

    onChange: (
        key: string,
        value: unknown
    ) => void;
};

export function CustomSettings({
    definitions,
    values,
    onChange,
}: CustomSettingsProps) {
    return (
        <div className="space-y-5">
            {definitions.map((setting) => {
                const value =
                    values[setting.key] ??
                    setting.default;

                switch (setting.type) {
                    case "text":
                        return (
                            <div
                                key={setting.key}
                                className="flex flex-col gap-2"
                            >
                                <Label
                                    htmlFor={setting.key}
                                    className="text-xs font-medium text-zinc-400"
                                >
                                    {setting.label}
                                </Label>

                                <Input
                                    id={setting.key}
                                    type="text"
                                    value={String(value ?? setting.default ?? "")}
                                    placeholder={setting.placeholder}
                                    onChange={(event) =>
                                        onChange(setting.key, event.target.value)
                                    }
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

                                {setting.description && (
                                    <p className="text-xs text-zinc-500">
                                        {setting.description}
                                    </p>
                                )}
                            </div>
                        );

                    case "number":
                        return (
                            <div
                                key={setting.key}
                                className="flex flex-col gap-2"
                            >
                                <Label
                                    htmlFor={setting.key}
                                    className="text-xs font-medium text-zinc-400"
                                >
                                    {setting.label}
                                </Label>

                                <Input
                                    id={setting.key}
                                    type="number"
                                    value={String(value ?? setting.default ?? "")}
                                    min={setting.min}
                                    max={setting.max}
                                    step={setting.step}
                                    onChange={(event) => {
                                        const nextValue = Number(event.target.value);

                                        if (Number.isNaN(nextValue)) {
                                            return;
                                        }

                                        onChange(setting.key, nextValue);
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

                                {setting.description && (
                                    <p className="text-xs text-zinc-500">
                                        {setting.description}
                                    </p>
                                )}
                            </div>
                        );

                    case "slider":
                        return (
                            <Slider
                                key={setting.key}
                                className="w-full"
                                value={Number(value ?? setting.default)}
                                minValue={setting.min}
                                maxValue={setting.max}
                                step={setting.step}
                                onChange={(value) => {
                                    const nextValue = Array.isArray(value)
                                        ? value[0]
                                        : value;

                                    onChange(setting.key, nextValue);
                                }}
                            >
                                <div className="flex items-center justify-between">
                                    <Label className="text-sm font-medium text-zinc-300">
                                        {setting.label}
                                    </Label>

                                    <Slider.Output className="text-sm text-zinc-500" />
                                </div>

                                <Slider.Track>
                                    <Slider.Fill />
                                    <Slider.Thumb />
                                </Slider.Track>

                                {setting.description && (
                                    <p className="mt-2 text-xs text-zinc-500">
                                        {setting.description}
                                    </p>
                                )}
                            </Slider>
                        );

                    case "switch":
                        return (
                            <div
                                key={setting.key}
                                className="flex items-center justify-between gap-8"
                            >
                                <div className="min-w-0">
                                    <h3 className="font-medium text-zinc-200">
                                        {setting.label}
                                    </h3>

                                    {setting.description && (
                                        <p className="mt-1 text-sm text-zinc-500">
                                            {setting.description}
                                        </p>
                                    )}
                                </div>

                                <Switch
                                    isSelected={Boolean(value ?? setting.default)}
                                    onChange={(nextValue) =>
                                        onChange(setting.key, nextValue)
                                    }
                                    aria-label={setting.label}
                                    className="group shrink-0"
                                >
                                    <Switch.Content>
                                        <Switch.Control>
                                            <Switch.Thumb />
                                        </Switch.Control>
                                    </Switch.Content>
                                </Switch>
                            </div>
                        );

                    case "select":
                        return (
                            <Select
                                className="w-full"
                                placeholder={setting.placeholder ?? "Select one"}
                                selectedKey={String(value ?? setting.default)}
                                onSelectionChange={(key) => {
                                    if (key !== null) {
                                        onChange(setting.key, String(key));
                                    }
                                }}
                            >
                                <Label>{setting.label}</Label>

                                <Select.Trigger>
                                    <Select.Value />
                                    <Select.Indicator />
                                </Select.Trigger>

                                <Select.Popover>
                                    <ListBox>
                                        {(setting.options ?? []).map((option) => (
                                            <ListBox.Item
                                                key={option.value}
                                                id={option.value}
                                                textValue={option.label}
                                            >
                                                {option.label}
                                                <ListBox.ItemIndicator />
                                            </ListBox.Item>
                                        ))}
                                    </ListBox>
                                </Select.Popover>
                            </Select>
                        );

                    case "color": {
                        const colorValue = String(
                            value ??
                            setting.default ??
                            "#ffffff"
                        );

                        const colorPresets = [
                            "#ef4444",
                            "#f97316",
                            "#eab308",
                            "#22c55e",
                            "#06b6d4",
                            "#3b82f6",
                            "#8b5cf6",
                            "#ec4899",
                            "#f43f5e",
                        ];

                        return (
                            <div
                                key={setting.key}
                                className="flex flex-col gap-2"
                            >
                                <ColorPicker
                                    value={parseColor(colorValue)}
                                    onChange={(color) =>
                                        onChange(
                                            setting.key,
                                            color.toString("hex")
                                        )
                                    }
                                >
                                    <ColorPicker.Trigger>
                                        <ColorSwatch size="lg" />
                                        <Label>{setting.label}</Label>
                                    </ColorPicker.Trigger>

                                    <ColorPicker.Popover className="gap-2 bg-[#191a1f]">
                                        <ColorSwatchPicker
                                            className="justify-center pt-2"
                                            size="xs"
                                        >
                                            {colorPresets.map((preset) => (
                                                <ColorSwatchPicker.Item
                                                    key={preset}
                                                    color={preset}
                                                >
                                                    <ColorSwatchPicker.Swatch />
                                                </ColorSwatchPicker.Item>
                                            ))}
                                        </ColorSwatchPicker>

                                        <ColorArea
                                            aria-label="Color area"
                                            className="max-w-full"
                                            colorSpace="hsb"
                                            xChannel="saturation"
                                            yChannel="brightness"
                                        >
                                            <ColorArea.Thumb />
                                        </ColorArea>

                                        <ColorSlider
                                            aria-label="Hue slider"
                                            channel="hue"
                                            className="px-1"
                                            colorSpace="hsb"
                                        >
                                            <ColorSlider.Track>
                                                <ColorSlider.Thumb />
                                            </ColorSlider.Track>
                                        </ColorSlider>

                                        <ColorField aria-label="Color field">
                                            <ColorField.Group variant="secondary" className="text-zinc-100 placeholder:text-zinc-600 bg-[#191a1f]">
                                                <ColorField.Prefix>
                                                    <ColorSwatch size="xs" />
                                                </ColorField.Prefix>

                                                <ColorField.Input />
                                            </ColorField.Group>
                                        </ColorField>
                                    </ColorPicker.Popover>
                                </ColorPicker>

                                {setting.description && (
                                    <p className="text-xs text-zinc-500">
                                        {setting.description}
                                    </p>
                                )}
                            </div>
                        );
                    }

                    default:
                        return null;
                }
            })}
        </div>
    );
}