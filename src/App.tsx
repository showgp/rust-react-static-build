import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import init, { add } from "wasm-lib";

function App() {
  const [ans, setAns] = useState(0);

  useEffect(() => {
    init().then(() => {
      setAns(add(1, 3));
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>To answer 1 + 3 = {ans}</p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
