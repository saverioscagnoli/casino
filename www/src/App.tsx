import { useState } from "react";

function App() {
  const [count, setCount] = useState<number>(0);

  return (
    // <BrowserRouter>
    //   <Routes>
    //     <Route path="/" element={<HomePage />} />
    //     <Route path="/session" element={<LoginPage />} />
    //   </Routes>
    // </BrowserRouter>
    <div>
      <p>Count: {count}</p>
      <button onClick={() => setCount(i => i + 1)}>increase</button>
    </div>
  );
}

export { App };
