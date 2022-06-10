import Button from "../../common/components/Button";
import React, { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useAppDispatch, useAppSelector } from "../../common/hooks";
import {
  selectCurrentContact,
  selectCurrentConversation,
  sendMessage,
} from "./chatSlice";
import { Message, MessageSource } from "../../services/chat";

export default function ChatPane() {
  const [messageInput, setMessageInput] = useState("");
  const currentContact = useAppSelector(selectCurrentContact);
  const currentConversation = useAppSelector(selectCurrentConversation);
  const dispatch = useAppDispatch();

  const submitMessage = async (e: FormEvent) => {
    e.preventDefault();
    try {
      dispatch(sendMessage({ pk: currentContact.pk, content: messageInput }));
      setMessageInput("");
    } catch (error) {
      console.log("error ", error);
    }
  };

  const renderMessage = (msg: Message) => {
    return msg.source == MessageSource.ME ? (
      <li className="bg-blue-1 text-white p-4 rounded-2xl self-end w-2/3">
        {msg.content}
      </li>
    ) : (
      <li className="bg-gray-1 text-white p-4 rounded-2xl self-start w-2/3">
        {msg.content}
      </li>
    );
  };

  return (
    <div className="flex-1 flex flex-col bg-black h-screen ">
      <div className="bg-gray-1 rounded p-2 m-4">
        <h1 className="text-2xl font-mono font-bold text-center">
          @{currentContact.alias}
        </h1>
        <h2 className="font-mono text-sm text-center break-all">
          {currentContact.pk}
        </h2>
      </div>
      <div className="flex flex-col flex-1 overflow-y-scroll">
        <ul className="space-y-4 px-4 flex flex-col ">
          {currentConversation.messages.map((msg) => renderMessage(msg))}
        </ul>
      </div>

      <form onSubmit={submitMessage} className="flex p-2">
        <input
          className="flex-1 mr-1"
          value={messageInput}
          placeholder="Say something nice"
          onChange={(e) => setMessageInput(e.target.value)}
        />
        <Button submit>Send</Button>
      </form>
    </div>
  );
}
