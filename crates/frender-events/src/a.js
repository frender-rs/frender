/**
 * Go to the following urls and run the code in console.
 *
 * - https://developer.mozilla.org/en-US/docs/Web/API/Element#events
 * - https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement#events
 * - https://developer.mozilla.org/en-US/docs/Web/API/DragEvent#event_types
 * - https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement#events
 * - https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement#events
 */
(() => {
  function firstToUpperCase(s) {
    if (!s) return s;
    return s[0].toUpperCase() + s.slice(1);
  }

  const EVENT_GROUPS = {
    // clipboard_events: { event: "ClipboardEvent" },
    composition_events: { event: "CompositionEvent" },
    focus_events: { event: "FocusEvent" },
    keyboard_events: { event: "KeyboardEvent" },
    mouse_events: { event: "MouseEvent" },
    touch_events: { event: "TouchEvent" },
    animation_events: { event: "AnimationEvent" },
    pointer_events: { event: "PointerEvent" },
    transition_events: { event: "TransitionEvent" },
  };

  const EVENT_TYPES = {
    securitypolicyviolation: {
      pascal: "SecurityPolicyViolation",
      event: "SecurityPolicyViolationEvent",
    },
    wheel: { event: "WheelEvent" },
    dblclick: { pascal: "DoubleClick" },
    beforeinput: { pascal: "BeforeInput", event: "InputEvent" },
  };

  const EVENT_PREFIXES = [
    "composition",
    "focus",
    "fullscreen",
    "key",
    "aux",
    "context",
    "mouse",
    "touch",
    // HTMLElement
    "animation",
    "LostPointer",
    "GotPointer",
    "pointer",
    "transition",
    "drag",
    // HTMLFormElement
    "FormData",
    // HTMLMediaElement
    "CanPlay",
    "duration",
    "loaded",
    "load",
    "rate",
    "time",
    "volume",
  ];

  function autoPascalEvent(eventName) {
    const override = EVENT_TYPES[eventName]?.pascal;
    if (override) {
      return override;
    }

    for (const prefix of EVENT_PREFIXES) {
      if (eventName.startsWith(prefix.toLowerCase())) {
        return (
          firstToUpperCase(prefix) +
          firstToUpperCase(eventName.slice(prefix.length))
        );
      }
    }

    return firstToUpperCase(eventName);
  }

  function childNodesToMarkdown(el) {
    return [...el.childNodes].map(elementToMarkdown).join("");
  }

  function elementToMarkdown(el) {
    if (el instanceof Text) {
      return el.data;
    }

    switch (el.tagName) {
      case "P":
        return childNodesToMarkdown(el) + "\n";
      case "A":
        return `[${childNodesToMarkdown(el)}](${el.href})`;
      case "KBD":
        return `<kbd>${childNodesToMarkdown(el)}</kbd>`;
      case "CODE":
        return `\`${el.innerText}\``;
      case "EM":
        return `*${childNodesToMarkdown(el)}*`;
      case "STRONG":
        return `**${childNodesToMarkdown(el)}**`;
      default:
        console.error("unknown element ", el);
        throw new Error(`unknown element <${el.tagName}>`);
    }
  }

  function getEvents() {
    return [...document.querySelectorAll(".main-page-content > section")]
      .filter((node) => {
        const label = node.getAttribute("aria-labelledby");
        return Boolean(label === "event_types" || label?.endsWith("events"));
      })
      .map((section) => {
        const h = section.querySelector("h3");
        const dts = [
          ...section.querySelectorAll(".section-content dl > dt"),
        ].filter(
          (dt) =>
            !dt.querySelector(
              ".icon-experimental, .icon-deprecated, .icon-nonstandard",
            ),
        );
        return (
          (h ? `\n// ${h.innerText}\n` : "") +
          dts
            .map((dt) => {
              const eventName = dt.innerText;
              const rustType = autoPascalEvent(eventName);
              const webSysEventType =
                EVENT_TYPES[eventName]?.event ||
                (h && EVENT_GROUPS[h.id]?.event) ||
                "Event";

              let dd = dt.nextElementSibling;
              if (dd.tagName !== "DD") {
                dd = null;
              }

              return (
                `/// Event ${childNodesToMarkdown(dt)}\n///\n` +
                (dd
                  ? childNodesToMarkdown(dd)
                      .trim()
                      .split("\n")
                      .filter(
                        (line) => !/^\s*Also available via the/.test(line),
                      )
                      .map((s) => `/// ${s}`)
                      .join("\n///\n") + "\n"
                  : "") +
                `${rustType}("${eventName}" : web_sys::${webSysEventType}),`
              );
            })
            .join("\n")
        );
      })
      .join("\n");
  }

  console.log(getEvents());
})();
