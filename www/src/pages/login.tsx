import { useState } from "react";
import { cn } from "~/lib/utils";

const LoginPage = () => {
  const [username, setUsername] = useState<string>("");

  const onLogin = async () => {
    let res = await fetch("http://127.0.0.1:3030/session", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username }),
    });

    if (res.ok) {
      let data = await res.json();
      console.log(data);
    }
  };

  return (
    <div
      className={cn("w-screen h-screen", "flex items-center justify-center")}
    >
      <div className={cn("flex flex-col gap-4")}>
        <input
          className={cn("border border-[var(--slate-7)] p-1 rounded")}
          placeholder="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <button className={cn("cursor-pointer")} onClick={onLogin}>
          Login
        </button>
      </div>
    </div>
  );
};

export { LoginPage };
