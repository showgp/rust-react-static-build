import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import init, { add, draw, julia } from "wasm-lib";

function generateRandomNumber() {
  const randomNumber = Math.random() * 0.4 + 0.3;
  const result = randomNumber.toFixed(2);
  return parseFloat(result);
}

function App() {
  const [ans, setAns] = useState(0);

  useEffect(() => {
    init().then(() => {
      setAns(add(1, 3));
      draw();
      // output
      const output = document.getElementById("output");
      const base64julia = julia(600, 600, -generateRandomNumber(), generateRandomNumber());
      const imgPath = "data:image/png;base64," + base64julia;
      output?.setAttribute("src", imgPath);
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
