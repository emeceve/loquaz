import React, { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import ChatList from "./components/ChatList";
import Nav from "./components/Nav";
import ChatPane from "./components/ChatPane";

function App() {
  return (
    <div className="flex">
      <Nav />
      <ChatList />
      <ChatPane />
    </div>
  );
}

export default App;
