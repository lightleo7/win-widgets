var media = (() => {
  const invoke = window.__TAURI_INTERNALS__.invoke.bind(
    window.__TAURI_INTERNALS__,
  );

  async function initializeSession(appId, appName) {
    await invoke("plugin:media|initialize_session", {
      request: {
        appId,
        appName,
      },
    });
  }

  async function setMetadata(metadata) {
    await invoke("plugin:media|set_metadata", {
      metadata,
    });
  }

  async function setPlaybackInfo(info) {
    await invoke("plugin:media|set_playback_info", {
      info,
    });
  }

  async function setPlaybackStatus(status) {
    await invoke("plugin:media|set_playback_status", {
      status,
    });
  }

  async function setPosition(position) {
    await invoke("plugin:media|set_position", {
      position,
    });
  }

  async function clearMetadata() {
    await invoke("plugin:media|clear_metadata");
  }

  async function getMetadata() {
    return await invoke("plugin:media|get_metadata");
  }

  async function getPlaybackInfo() {
    return await invoke("plugin:media|get_playback_info");
  }

  async function getPlaybackStatus() {
    return await invoke("plugin:media|get_playback_status");
  }

  async function getPosition() {
    return await invoke("plugin:media|get_position");
  }

  async function isEnabled() {
    return await invoke("plugin:media|is_enabled");
  }

  async function next() {
    await invoke("plugin:media|next");
  }

  async function previous() {
    await invoke("plugin:media|previous");
  }

  return {
    initializeSession,

    setMetadata,
    setPlaybackInfo,
    setPlaybackStatus,
    setPosition,

    clearMetadata,

    getMetadata,
    getPlaybackInfo,
    getPlaybackStatus,
    getPosition,

    isEnabled,

    next,
    previous,

    Playing: "playing",
    Paused: "paused",
    Stopped: "stopped",

    RepeatNone: "none",
    RepeatTrack: "track",
    RepeatList: "list",
  };
})();
