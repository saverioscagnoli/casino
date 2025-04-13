import { useCallback, useEffect, useRef, useState } from "react";

type ReadyState = "Connecting" | "Open" | "Closing" | "Closed";

const StateMap: { [key: number]: ReadyState } = {
  [WebSocket.CONNECTING]: "Connecting",
  [WebSocket.OPEN]: "Open",
  [WebSocket.CLOSING]: "Closing",
  [WebSocket.CLOSED]: "Closed",
};

type UseSocketReturn = {
  send<T>(data: T): void;
  state: ReadyState;
  lastMessage: MessageEvent | null;
  sendError: Error | null;
};

type WebSocketSendData = string | ArrayBufferLike | Blob | ArrayBufferView;

function useSocket(
  url: string,
  protocols?: string | string[]
): UseSocketReturn {
  const [lastMessage, setLastMessage] = useState<MessageEvent | null>(null);
  const [state, setState] = useState<ReadyState>(StateMap[WebSocket.CLOSED]);
  const [sendError, setSendError] = useState<Error | null>(null);

  const ws = useRef<WebSocket | null>(null);

  const protocolsRef = useRef<string | string[] | undefined>(protocols);

  useEffect(() => {
    protocolsRef.current = protocols;
  }, [protocols]);

  useEffect(() => {
    let socket = new WebSocket(url, protocolsRef.current);

    ws.current = socket;
    setState(StateMap[socket.readyState]);

    let controller = new AbortController();
    let { signal } = controller;

    socket.addEventListener(
      "open",
      () => {
        setState(StateMap[socket.readyState]);
      },
      { signal }
    );

    socket.addEventListener(
      "close",
      () => {
        setState(StateMap[socket.readyState]);
      },
      { signal }
    );

    socket.addEventListener(
      "message",
      (event) => {
        setLastMessage(event);
      },
      { signal }
    );

    socket.addEventListener(
      "error",
      (event) => {
        console.error("WebSocket error:", event);
      },
      { signal }
    );

    return () => {
      controller.abort();

      if (
        socket.readyState === WebSocket.OPEN ||
        socket.readyState === WebSocket.CONNECTING
      ) {
        socket.close(1000, "Hook unmounting");
      }
    };
  }, [url]);

  const send = useCallback(
    <T>(data: T) => {
      let socket = ws.current;

      if (socket?.readyState !== WebSocket.OPEN) {
        console.warn("Socket is not open. Cannot send message.", {
          url,
          state: StateMap[socket?.readyState ?? WebSocket.CLOSED],
        });
        return;
      }

      try {
        // Handle the data based on its type
        if (
          typeof data === "string" ||
          data instanceof ArrayBuffer ||
          data instanceof Blob ||
          ArrayBuffer.isView(data)
        ) {
          // These types can be sent directly to WebSocket
          socket.send(data as WebSocketSendData);
        } else {
          // For other types, convert to JSON string
          socket.send(JSON.stringify(data));
        }
      } catch (error) {
        setSendError(error as Error);
        console.error("Error sending message:", error);
      }
    },
    [url]
  );

  return {
    send,
    state,
    lastMessage,
    sendError,
  };
}

export { useSocket };
export type { UseSocketReturn, ReadyState };
