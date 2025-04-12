import { BrowserRouter, Routes, Route } from "react-router";
import { LoginPage } from "./pages/login";
import { HomePage } from "./pages/home";

const App = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route index path="/" element={<LoginPage />} />
        <Route path="/home" element={<HomePage />} />
      </Routes>
    </BrowserRouter>
  );
};

export { App };
