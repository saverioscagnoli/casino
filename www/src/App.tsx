import { useEffect, useMemo, useState } from "react";
import { TextOpcode } from "./types/opcdode";
import {
  ChatMessagePayload,
  HelloPayload,
  UnknownTextPayload,
} from "./types/payload";

const App = () => {
  const ws = useMemo(() => {
    let ws = new WebSocket("ws://localhost:3030");
    ws.binaryType = "arraybuffer";
    return ws;
  }, []);

  const [id, setID] = useState<string>("");
  const [msg, setMsg] = useState<string>("");

  const [messages, setMessages] = useState<ChatMessagePayload[]>([]);

  useEffect(() => {
    if (id !== "") {
      console.log("id:", id);
    }
  }, [id]);

  useEffect(() => {
    ws.addEventListener("open", () => console.log("socket open"));
    ws.addEventListener("close", () => console.log("socket closed"));

    ws.addEventListener("error", console.error);

    ws.addEventListener("message", (event) => {
      let parsed: UnknownTextPayload = JSON.parse(event.data);

      switch (parsed.opcode) {
        case TextOpcode.Hello: {
          let data = parsed.data as HelloPayload;

          setID(data.id);
          break;
        }

        case TextOpcode.ChatMessage: {
          let data = parsed.data as ChatMessagePayload;
          setMessages((m) => [...m, data]);
          break;
        }
      }
    });
  }, [ws, setID]);

  return (
    <div>
      <input onChange={(e) => setMsg(e.target.value)} />
      <button
        onClick={() => {
          let message: ChatMessagePayload = {
            authorID: id,
            content: msg,
          };

          ws.send(
            JSON.stringify({
              opcode: TextOpcode.ChatMessage,
              data: message,
            })
          );

          setMessages((m) => [...m, message]);
        }}
      >
        Send
      </button>

      <div>
        {messages.map(({ authorID, content }, index) => (
          <div key={index.toString()}>
            <p>{authorID}:</p>
            <p>{content}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export { App };
