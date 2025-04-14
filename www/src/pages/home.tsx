import { useEffect } from "react";
import { useLocation } from "react-router";
import { useSocket } from "~/hooks/use-socket";
import { Decoder, Encoder } from "~/lib/utils";
import { Opcode } from "~/types/payload";

const HomePage = () => {
  const location = useLocation();
  const username = location.state as string;

  const { lastMessage, ...socket } = useSocket("ws://localhost:3030");

  useEffect(() => {
    switch (socket.state) {
      case "Open": {
        console.log("Socket opened, sending hello payload...");

        let usernameBuffer = Encoder.encode(username);
        let buffer = new Uint8Array(1 + usernameBuffer.length);

        // Set Opcode+username
        // Example [0, "helloworld!!" (as bytes)]
        buffer[0] = Opcode.Hello;
        buffer.set(usernameBuffer, 1);

        socket.send(buffer);
        break;
      }

      case "Closed": {
        console.log("Socket closed");
        break;
      }
    }
  }, [socket.state, username]);

  useEffect(() => {
    if (!lastMessage) {
      return;
    }

    let data = lastMessage.data;

    if (!(data instanceof ArrayBuffer)) {
      return;
    }

    let buffer = new Uint8Array(data);
    let opcode = buffer[0];

    switch (opcode) {
      case Opcode.Hello: {
        let encoded = buffer.slice(1);
        let id = Decoder.decode(encoded);
        
        console.log(id);
        break;
      }
    }
  }, [lastMessage]);

  return <div>HomePage</div>;
};

export { HomePage };
