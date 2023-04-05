import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [poem, setPoem] = useState("");
  const [sanitizedPoem, setSanitizedPoem] = useState("");
  const [title, setTitle] = useState("");

  async function retrieve() {
    setPoem(await invoke("get_poem", { title }));
  }

  useEffect(()=>{
    setSanitizedPoem(poem.replaceAll('","'," ").replaceAll(/\\/g,""))
  },[poem])
  return (
    <div className="container">
      <h1>John Keats</h1>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            retrieve();
          }}
        >
          <input
            id="title-input"
            onChange={(e) => setTitle(e.currentTarget.value)}
            placeholder="Search by title"
          />
          <button type="submit">Search</button>
        </form>
      </div>
      <p>{sanitizedPoem}</p>
    </div>
  );
}

export default App;
