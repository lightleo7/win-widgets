// tauri.js

var tauri = (() => {
  const internals = window.__TAURI_INTERNALS__;

    return {
      invoke: internals.invoke.bind(internals),
      getSettings() {
        return internals.invoke("get_current_widget_settings");
      }
    };
})();
