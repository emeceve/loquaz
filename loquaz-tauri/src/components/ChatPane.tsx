import Button from "./Button";
import React, { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

export default function ChatPane() {
  const [messageInput, setMessageInput] = useState("");
  const [responseContainer, setResponsecontainer] = useState("");

  function updateResponse(response: any) {
    setResponsecontainer(
      typeof response === "string" ? response : JSON.stringify(response)
    );
  }

  const submitMessage = async (e: FormEvent) => {
    e.preventDefault();
    // e.preventDefault();
    try {
      const response = await invoke("message", {
        value: messageInput,
      });
      console.debug("response addsfdsf ", response);
      updateResponse(response);
      setMessageInput("");
    } catch (error) {
      console.log("error ", error);
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-black h-screen">
      <div className="flex flex-col flex-1">
        <div className="bg-gray-1 rounded p-2 m-4">
          <h1 className="text-2xl font-mono font-bold text-center">@fiatjaf</h1>
          <h2 className="font-mono text-sm text-center break-all">
            b8f4f63930574e90bfb240331b19530c958c3b0d65ed7df099b717adc6215d55
          </h2>
        </div>
        <ul className="space-y-4 px-4 flex flex-col">
          <li className="bg-gray-1 text-white p-4 rounded-2xl self-start w-2/3">
            fake message
          </li>
          <li className="bg-blue-1 text-white p-4 rounded-2xl self-end w-2/3">
            {responseContainer}
          </li>
        </ul>
      </div>

      <form onSubmit={submitMessage} className="flex p-2">
        <input
          className="flex-1 mr-1"
          value={messageInput}
          placeholder="Say something nice"
          onChange={(e) => setMessageInput(e.target.value)}
        />
        <Button submit>Send bla</Button>
      </form>
    </div>
  );
}
