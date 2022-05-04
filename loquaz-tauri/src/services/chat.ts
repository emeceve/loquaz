import { invoke } from "@tauri-apps/api";
import { Contact } from "./config";

export interface Conversation {
  contact: Contact;
  messages: Message[];
}
export interface Message {
  content: string;
  source: MessageSource;
  ev: any;
}
export enum MessageSource {
  ME = "Me",
  THEN = "Then",
}
export async function getConversation(pk: string): Promise<Conversation> {
  return await invoke("get_conversation", { pk });
}
