import { BrowserRouter, Route, Routes } from "react-router";
import { LoginPage } from "~/pages/login";
import { HomePage } from "~/pages/home";
import { useUser } from "./states/user";
import { useEffect } from "react";

function App() {
  const user = useUser();

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/session" element={<LoginPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export { App };
