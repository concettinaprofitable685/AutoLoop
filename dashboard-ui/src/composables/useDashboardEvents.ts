import { onBeforeUnmount, ref } from "vue";

export function useDashboardEvents() {
  const connected = ref(false);
  const lastEvent = ref<Record<string, unknown> | null>(null);
  let source: EventSource | null = null;

  function connect(baseUrl: string, onMessage: (payload: Record<string, unknown>) => void) {
    disconnect();
    source = new EventSource(`${baseUrl.replace(/\/$/, "")}/api/events`);
    source.addEventListener("connected", () => {
      connected.value = true;
    });
    source.addEventListener("dashboard", (event) => {
      try {
        const payload = JSON.parse((event as MessageEvent).data) as Record<string, unknown>;
        lastEvent.value = payload;
        onMessage(payload);
      } catch {
        // Ignore malformed events from the lightweight stream.
      }
    });
    source.onerror = () => {
      connected.value = false;
    };
  }

  function disconnect() {
    if (source) {
      source.close();
      source = null;
    }
    connected.value = false;
  }

  onBeforeUnmount(disconnect);

  return {
    connected,
    lastEvent,
    connect,
    disconnect
  };
}
