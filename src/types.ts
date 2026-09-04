// export type TabKey =
//   | "widgets"
//   | "settings"
//   | "discover";

// export type WidgetSettings = Record<string, unknown>;

// export type Widget = {
//   // Тип виджета / имя его папки.
//   // Например: "clock", "weather", "sys_mon"
//   id: string;

//   // Уникальный label конкретного экземпляра окна.
//   // Например: "clock-1", "clock-2"
//   windowLabel: string;

//   enabled: boolean;

//   width: number;
//   height: number;

//   x: number;
//   y: number;

//   interactive: boolean;

//   settings?: WidgetSettings;

//   settingDefinitions?: WidgetSettingDefinition[];
// };

// export type WidgetSettingType =
//   | "text"
//   | "number"
//   | "slider"
//   | "switch"
//   | "select"
//   | "color";

// export type WidgetSettingOption = {
//   label: string;
//   value: string;
// };

// export type WidgetSettingDefinition = {
//   key: string;
//   label: string;
//   type: WidgetSettingType;

//   default: unknown;

//   description?: string;

//   placeholder?: string;

//   min?: number;
//   max?: number;
//   step?: number;

//   options?: WidgetSettingOption[];
// };

export type TabKey =
  | "widgets"
  | "settings"
  | "discover";

export type WidgetSettings =
  Record<string, unknown>;

export type Widget = {
  id: string;
  windowLabel: string;

  name: string;
  version: string;
  author: string;
  description: string;

  enabled: boolean;

  width: number;
  height: number;

  x: number;
  y: number;

  interactive: boolean;

  settings?: WidgetSettings;

  settingDefinitions: WidgetSettingDefinition[];
};

export type WidgetSettingType =
  | "text"
  | "number"
  | "slider"
  | "switch"
  | "select"
  | "color";

export type WidgetSettingOption = {
  label: string;
  value: string;
};

export type WidgetSettingDefinition = {
  key: string;
  label: string;

  type: WidgetSettingType;

  default: unknown;

  description?: string;

  placeholder?: string;

  min?: number;
  max?: number;
  step?: number;

  options?: WidgetSettingOption[];
};