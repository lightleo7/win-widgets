import {
  useMemo,
  useState,
  useEffect,
} from "react";

import type {
  TabKey,
  Widget,
} from "./types";

import {
  getWidgets,
} from "./data/widgets";

import {
  AppHeader,
} from "./components/AppHeader";

import {
  AppNavigation,
} from "./components/AppNavigation";

import {
  WidgetsPage,
} from "./components/WidgetsPage";

import {
  SettingsPage,
} from "./components/SettingsPage";

import {
  DiscoverPage,
  type DownloadedPack,
} from "./components/DiscoverPage";

import {
  invoke,
} from "@tauri-apps/api/core";

import {
  enable,
  disable,
  isEnabled,
} from "@tauri-apps/plugin-autostart";

function App() {
  const [activeTab, setActiveTab] =
    useState<TabKey>("widgets");

  const [widgets, setWidgets] =
    useState<Widget[]>([]);

  const [
    selectedWidgetId,
    setSelectedWidgetId,
  ] = useState<string>("");

  const [
    startWithWindows,
    setStartWithWindows,
  ] = useState(false);

  const [
    restoreWidgets,
    setRestoreWidgets,
  ] = useState(true);

  const [widgetUrl, setWidgetUrl] = useState("");

  const [downloadedPack, setDownloadedPack] =
    useState<DownloadedPack | null>(null);

  const [selectedWidgets, setSelectedWidgets] =
    useState<string[]>([]);

  const [isDownloadingPack, setIsDownloadingPack] =
    useState(false);

  const [isInstallingPack, setIsInstallingPack] =
    useState(false);

  const [installError, setInstallError] =
    useState<string | null>(null);


  async function handleStartWithWindows(value: boolean) {
    if (value) {
      await enable();
    } else {
      await disable();
    }

    setStartWithWindows(value);
  }

  useEffect(() => {
    isEnabled().then(setStartWithWindows);
  }, []);


  useEffect(() => {
    getWidgets()
      .then((loadedWidgets) => {
        setWidgets(loadedWidgets);

        if (loadedWidgets.length > 0) {
          setSelectedWidgetId(
            loadedWidgets[0].windowLabel
          );
        }
      })
      .catch(console.error);
  }, []);

  const selectedWidget =
    useMemo(() => {
      return (
        widgets.find(
          (widget) =>
            widget.windowLabel === selectedWidgetId
        )
        ?? widgets[0]
      );
    }, [
      widgets,
      selectedWidgetId,
    ]);

  async function updateWidget(
    windowLabel: string,
    patch: Partial<Widget>
  ) {
    const currentWidget =
      widgets.find(
        (widget) =>
          widget.windowLabel === windowLabel
      );

    if (!currentWidget) {
      return;
    }

    const updatedWidget: Widget = {
      ...currentWidget,
      ...patch,
    };

    // Сначала обновляем UI.
    setWidgets((currentWidgets) =>
      currentWidgets.map((widget) =>
        widget.windowLabel === windowLabel
          ? updatedWidget
          : widget
      )
    );

    try {
      await invoke("save_widget", {
        widget: updatedWidget,
      });
    } catch (error) {
      console.error(
        "Failed to save widget:",
        error
      );
    }
  }

  async function updateWidgetSetting(
    windowLabel: string,
    key: string,
    value: unknown
  ) {
    const widget = widgets.find(
      (widget) =>
        widget.windowLabel === windowLabel
    );

    if (!widget) {
      return;
    }

    const settings = {
      ...widget.settings,
      [key]: value,
    };

    setWidgets((currentWidgets) =>
      currentWidgets.map((widget) =>
        widget.windowLabel === windowLabel
          ? {
            ...widget,
            settings,
          }
          : widget
      )
    );

    try {
      await invoke(
        "save_widget_settings",
        {
          windowLabel,
          settings,
        }
      );
    } catch (error) {
      console.error(
        "Failed to save settings:",
        error
      );
    }
  }

  async function reloadWidgets() {
    console.log(
      "reload widgets"
    );

    await invoke("reload_widgets");

    const loadedWidgets =
      await getWidgets();

    setWidgets(loadedWidgets);

    if (
      loadedWidgets.length > 0 &&
      !loadedWidgets.some(
        (widget) =>
          widget.windowLabel === selectedWidgetId
      )
    ) {
      setSelectedWidgetId(
        loadedWidgets[0].windowLabel
      );
    }
  }

  async function downloadWidgetPack() {
    if (!widgetUrl.trim()) {
      return;
    }

    try {
      setIsDownloadingPack(true);

      setInstallError(null);

      setDownloadedPack(null);

      setSelectedWidgets([]);

      const pack = await invoke<DownloadedPack>(
        "download_widget_pack",
        {
          repository: widgetUrl.trim(),
        }
      );

      setDownloadedPack(pack);

      // По умолчанию выбираем все найденные виджеты.
      setSelectedWidgets(
        pack.widgets.map(
          (widget) => widget.relativePath
        )
      );

    } catch (error) {

      console.error(
        "Failed to download widget pack:",
        error
      );

      setInstallError(
        String(error)
      );

    } finally {

      setIsDownloadingPack(false);

    }
  }

  async function installSelectedWidgets() {
    if (
      !downloadedPack
      || selectedWidgets.length === 0
    ) {
      return;
    }

    try {
      setIsInstallingPack(true);

      setInstallError(null);

      const installed = await invoke<string[]>(
        "install_widgets_from_pack",
        {
          request: {
            token: downloadedPack.token,

            author: downloadedPack.author,

            pack: downloadedPack.pack,

            widgets: selectedWidgets,
          },
        }
      );

      console.log(
        "Installed widgets:",
        installed
      );

      // Обновляем список виджетов в приложении.
      await reloadWidgets();

      // Закрываем текущий результат.
      setDownloadedPack(null);

      setSelectedWidgets([]);

      setWidgetUrl("");

    } catch (error) {

      console.error(
        "Failed to install widgets:",
        error
      );

      setInstallError(
        String(error)
      );

    } finally {

      setIsInstallingPack(false);

    }
  }

  async function handleReloadWidget(widgetId: string) {
    try {
      await invoke("reload_widget", {
        widgetId,
      });
    } catch (error) {
      console.error(
        `Failed to reload widget ${widgetId}:`,
        error
      );
    }
  }

  async function handleDeleteWidget(widgetId: string) {
    try {
      await invoke("remove_widget", {
        widgetId,
      });

      await reloadWidgets();
    } catch (error) {
      console.error(
        `Failed to remove widget ${widgetId}:`,
        error
      );
    }
  }

  async function handleResetWidgetSettings(
    widgetId: string,
    definitions: Widget["settingDefinitions"]
  ) {
    const widget = widgets.find(
      (widget) => widget.id === widgetId
    );

    if (!widget) {
      return;
    }

    const settings = Object.fromEntries(
      definitions.map((setting) => [
        setting.key,
        setting.default,
      ])
    );

    // Сразу обновляем UI.
    setWidgets((currentWidgets) =>
      currentWidgets.map((currentWidget) =>
        currentWidget.id === widgetId
          ? {
            ...currentWidget,
            settings,
          }
          : currentWidget
      )
    );

    try {
      await invoke("save_widget_settings", {
        windowLabel: widget.windowLabel,
        settings,
      });

      await handleReloadWidget(widgetId);
    } catch (error) {
      console.error(
        `Failed to reset settings for ${widgetId}:`,
        error
      );
    }
  }

  return (
    <main className="min-h-screen bg-[#0b0c0f] text-zinc-100 dark">
      <AppHeader
        onReload={reloadWidgets}
      />

      <div className="mx-auto max-w-[1400px] px-8 py-6">
        <AppNavigation
          activeTab={activeTab}
          onChange={setActiveTab}
        />

        <div className="pt-8">
          {(activeTab === "widgets" && !selectedWidget) && (
            <div className="flex min-h-[500px] items-center justify-center text-zinc-500">
              No widgets installed
            </div>
          )}
          {(activeTab === "widgets" && selectedWidget) && (
            <WidgetsPage
              widgets={widgets}

              selectedWidget={
                selectedWidget
              }

              onSelectWidget={
                setSelectedWidgetId
              }

              onUpdateWidget={
                updateWidget
              }

              onUpdateSetting={
                updateWidgetSetting
              }
              onResetWidget={handleResetWidgetSettings}
              onReloadWidget={handleReloadWidget}
              onDeleteWidget={handleDeleteWidget}
            />
          )}

          {activeTab === "settings" && (
            <SettingsPage
              startWithWindows={
                startWithWindows
              }

              restoreWidgets={
                restoreWidgets
              }

              onStartWithWindowsChange={
                handleStartWithWindows
              }

              onRestoreWidgetsChange={
                setRestoreWidgets
              }
            />
          )}

          {activeTab === "discover" && (
            <DiscoverPage
              widgetUrl={widgetUrl}
              onWidgetUrlChange={setWidgetUrl}

              onInstall={downloadWidgetPack}

              isLoading={isDownloadingPack}

              pack={downloadedPack}

              selectedWidgets={selectedWidgets}

              onSelectedWidgetsChange={setSelectedWidgets}

              onInstallSelected={
                installSelectedWidgets
              }

              isInstalling={isInstallingPack}

              error={installError}
            />
          )}
        </div>
      </div>
    </main>
  );
}

export default App;