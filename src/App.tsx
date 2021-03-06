import React, { FormEvent, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import ChatList from "./features/chat/ChatList";
import Nav from "./common/components/Nav";
import ChatPage from "./features/chat/ChatPage";
import { Route, Routes } from "react-router-dom";
import ConfigPage from "./features/config/ConfigPage";
import { getConfig } from "./services/config";
import { useDispatch } from "react-redux";
import { loadConfig, updatedConfig } from "./features/config/configSlice";
import { useAppDispatch } from "./common/hooks";
import { receivedNewMessage } from "./features/chat/chatSlice";
import { Message } from "./services/chat";

function App() {
  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(loadConfig());

    listen<Message>("new_message", (ev) => {
      dispatch(receivedNewMessage(ev.payload));
    }).catch(console.log);
  }, []);
  return (
    <div className="flex">
      <Nav />
      <Routes>
        <Route path="/" element={<ChatPage />} />
        <Route path="/config" element={<ConfigPage />} />
      </Routes>
    </div>
  );
}

export default App;
