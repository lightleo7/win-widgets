// error_handler.js

(function () {
  "use strict";

  const ERROR_HANDLER = {
    initialized: false,

    overlay: null,

    message: null,

    errors: [],

    init() {
      if (this.initialized) {
        return;
      }

      this.initialized = true;

      this.createStyles();
      this.createOverlay();
      this.bindEvents();
    },

    createStyles() {
      if (document.getElementById("widget-error-handler-styles")) {
        return;
      }

      const style = document.createElement("style");

      style.id = "widget-error-handler-styles";

      style.textContent = `
        #widget-error-overlay {
          position: fixed;

          inset: 0;

          z-index: 2147483647;

          display: flex;
          align-items: center;
          justify-content: center;

          padding: 8px;

          background:
            rgba(45, 0, 0, 0.88);

          backdrop-filter: blur(10px);
          -webkit-backdrop-filter: blur(10px);

          box-shadow: none;
        }


        #widget-error-overlay[hidden] {
          display: none;
        }


        .widget-error-window {
          width: 100%;
          height: 100%;

          display: flex;
          flex-direction: column;

          overflow: hidden;

          border:
            1px solid rgba(255, 85, 85, 0.5);

          border-radius:
            clamp(10px, 6vw, 18px);

          background:
            rgba(55, 12, 14, 0.98);

          color:
            rgba(255, 255, 255, 0.96);

          box-shadow: none;

          font-family:
            -apple-system,
            BlinkMacSystemFont,
            "Segoe UI",
            sans-serif;
        }


        .widget-error-header {
          min-height: 38px;

          padding: 8px 10px;

          display: flex;
          align-items: center;
          justify-content: space-between;

          flex-shrink: 0;

          border-bottom:
            1px solid
            rgba(255, 100, 100, 0.15);
        }


        .widget-error-title {
          display: flex;
          align-items: center;

          gap: 7px;

          min-width: 0;

          font-size:
            clamp(9px, 3vw, 11px);

          font-weight: 700;

          letter-spacing: 1px;

          color:
            rgba(255, 125, 125, 0.96);

          white-space: nowrap;

          overflow: hidden;
          text-overflow: ellipsis;
        }


        .widget-error-dot {
          width: 7px;
          height: 7px;

          flex-shrink: 0;

          border-radius: 50%;

          background: #ff5c5c;

          box-shadow: none;
        }


        .widget-error-close {
          width: 24px;
          height: 24px;

          flex-shrink: 0;

          display: grid;
          place-items: center;

          padding: 0;

          border:
            1px solid
            rgba(255, 255, 255, 0.08);

          border-radius: 7px;

          background:
            rgba(255, 255, 255, 0.05);

          color:
            rgba(255, 255, 255, 0.75);

          font-size: 17px;
          line-height: 1;

          cursor: pointer;

          box-shadow: none;
        }


        .widget-error-close:hover {
          background:
            rgba(255, 255, 255, 0.1);
        }


        .widget-error-message {
          flex: 1;

          min-height: 0;

          overflow: auto;

          padding: 10px;

          font-family:
            "SF Mono",
            "Cascadia Code",
            Consolas,
            monospace;

          font-size:
            clamp(9px, 3vw, 12px);

          line-height: 1.45;

          white-space: pre-wrap;

          overflow-wrap: anywhere;

          color:
            rgba(255, 215, 215, 0.96);
        }


        @media (max-height: 120px) {

          #widget-error-overlay {
            padding: 4px;
          }

          .widget-error-header {
            min-height: 28px;
            padding: 4px 7px;
          }

          .widget-error-close {
            width: 20px;
            height: 20px;
          }

          .widget-error-message {
            padding: 6px;
          }

        }

      `;

      document.head.appendChild(style);
    },

    createOverlay() {
      const overlay = document.createElement("div");

      overlay.id = "widget-error-overlay";

      overlay.hidden = true;

      overlay.innerHTML = `
        <div class="widget-error-window">
          <div class="widget-error-header">
            <div class="widget-error-title">
              <span
                class="widget-error-dot"
              ></span>
              JS WIDGET ERROR
            </div>

            <button
              class="widget-error-close"
              type="button"
              aria-label="Закрыть ошибку"
            >
              ×
            </button>
          </div>
          <div
            class="widget-error-message"
          ></div>
        </div>
      `;

      document.body.appendChild(overlay);

      this.overlay = overlay;

      this.message = overlay.querySelector(".widget-error-message");

      const closeButton = overlay.querySelector(".widget-error-close");

      closeButton.addEventListener("click", () => {
        this.hide();
      });
    },

    bindEvents() {
      window.addEventListener("error", (event) => {
        this.show(event.error || event.message, {
          source: event.filename,
          line: event.lineno,
          column: event.colno,
        });
      });

      window.addEventListener("unhandledrejection", (event) => {
        this.show(event.reason, {
          source: "Unhandled Promise Rejection",
        });
      });
    },

    formatError(error, options = {}) {
      const { source = "", line = null, column = null } = options;

      let output = "";

      if (error instanceof Error) {
        output += `${error.name}: ${error.message}`;
        if (error.stack) {
          output += `\n\n${error.stack}`;
        }
      } else if (typeof error === "object" && error !== null) {
        try {
          output += JSON.stringify(error, null, 2);
        } catch {
          output += String(error);
        }
      } else {
        output += String(error);
      }

      if (source) {
        output += `\n\nSource: ${source}`;
      }

      if (line !== null && column !== null) {
        output += `:${line}:${column}`;
      } else if (line !== null) {
        output += `\nLine: ${line}`;
      } else if (column !== null) {
        output += `\nColumn: ${column}`;
      }

      return output;
    },

    show(error, options = {}) {
      if (!this.initialized) {
        this.init();
      }

      const formatted = this.formatError(error, options);

      this.errors.push({
        error,
        options,
        time: Date.now(),
      });

      this.message.textContent = formatted;

      this.overlay.hidden = false;
    },

    hide() {
      if (!this.overlay) {
        return;
      }

      this.overlay.hidden = true;
    },

    clear() {
      this.errors = [];

      if (this.message) {
        this.message.textContent = "";
      }
    },

    getErrors() {
      return [...this.errors];
    },
  };

  window.WidgetErrorHandler = ERROR_HANDLER;

  window.debugError = function (error, source = "") {
    ERROR_HANDLER.show(error, {
      source,
    });
  };

  window.onerror = function (message, source, line, column, error) {
    WidgetErrorHandler.show(error || message, {
      source,
      line,
      column,
    });

    return true;
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => {
      ERROR_HANDLER.init();
    });
  } else {
    ERROR_HANDLER.init();
  }
})();
