import { useLocation } from "react-router";
import { useSocket } from "../hooks/use-socket";

const HomePage = () => {
  const location = useLocation();
  const username = location.state as string;

  const { send, onOpen, onClose, onMessage } = useSocket("ws://localhost:3030");

  onOpen(() => {
    send({})
  })

  onMessage((event) => console.log(event.data));

  return <div>HomePage</div>;
};

export { HomePage };
