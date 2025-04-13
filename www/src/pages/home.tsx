import { useEffect } from "react";
import { useLocation } from "react-router";
import { useSocket } from "~/hooks/use-socket";

const HomePage = () => {
  const location = useLocation();
  const username = location.state as string;

  const { lastMessage, ...socket } = useSocket("ws://localhost:3030");

  useEffect(() => {
    switch (socket.state) {
      case "Open": {
        console.log("Socket opened");

        const opcode = 0;
        const usernameBuffer = new TextEncoder().encode(username);
        const buffer = new Uint8Array(1 + usernameBuffer.length);
        buffer[0] = opcode; // Set the first byte as the opcode
        buffer.set(usernameBuffer, 1); // Append the username bytes
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

    console.log("Received message:", lastMessage);
  }, [lastMessage]);

  return <div>HomePage</div>;
};

export { HomePage };
