import { useEffect, useMemo } from "react";

function useSocket(url: string) {
  const socket = useMemo(() => {
    let ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    return ws;
  }, []);

  const send = <T>(data: T | string | ArrayBuffer) => {
    if (socket.readyState !== WebSocket.OPEN) {
      console.error("Socket is not open");
      return;
    }

    if (typeof data === "string") {
      socket.send(data);
    } else if (data instanceof ArrayBuffer) {
      socket.send(data);
    } else {
      socket.send(JSON.stringify(data));
    }
  };

  const close = () => {
    if (socket.readyState === WebSocket.OPEN) {
      socket.close();
    }
  };

  const onOpen = (callback: () => void) => {
    useEffect(() => {
      let ctrl = new AbortController();
      let { signal } = ctrl;

      socket.addEventListener("open", callback, { signal });

      return () => {
        ctrl.abort();
      };
    }, []);
  };

  const onClose = (callback: () => void) => {
    useEffect(() => {
      let ctrl = new AbortController();
      let { signal } = ctrl;

      socket.addEventListener("close", callback, { signal });

      return () => {
        ctrl.abort();
      };
    }, []);
  };

  const onMessage = (callback: (event: MessageEvent) => void) => {
    useEffect(() => {
      let ctrl = new AbortController();
      let { signal } = ctrl;

      socket.addEventListener("message", callback, { signal });

      return () => {
        ctrl.abort();
      };
    }, []);
  };

  return {
    socket,
    send,
    close,
    onOpen,
    onClose,
    onMessage,
  };
}

export { useSocket };
