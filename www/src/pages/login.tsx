import { useState } from "react";
import { useNavigate } from "react-router";

const LoginPage = () => {
  const nav = useNavigate();
  const [username, setUsername] = useState<string>("");

  const onLogin = () => {
    if (username.trim() === "") {
      alert("Please enter a username");
      return;
    }

    nav("/home", { state: username });
  };

  return (
    <div>
      <input
        type="text"
        placeholder="Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <button onClick={onLogin}>Login</button>
    </div>
  );
};

export { LoginPage };
