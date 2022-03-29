import React, { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {listen} from '@tauri-apps/api/event'

import ChatList from "./components/ChatList";
import Nav from "./components/Nav";
import ChatPage from "./components/ChatPage";
import {Route, Routes} from "react-router-dom";
import ConfigPage from "./components/ConfigPage";

function App() {
 const unlisten = listen('test-event', ev => {
  console.log("Event received");
  console.log(ev)
}).then();

  return (
    <div className="flex">
      <Nav />
      <Routes>
        <Route path="/" element={<ChatPage/>} />
        <Route path="/config" element={<ConfigPage/>} />
      </Routes>
    </div>
  );
}

export default App;
