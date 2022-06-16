import { invoke } from "@tauri-apps/api";

export async function getConfig() {
  const [relays, contacts] = await invoke("get_config");
  return {
    relays,
    contacts,
  };
}

export interface Contact {
  alias: string;
  pk: string;
}

export async function addContact(contact: Contact) {
  const res = await invoke("add_contact", { ...contact });
}

export async function removeContact(contact: Contact) {
  await invoke("remove_contact", { contact });
}

export async function addRelay(url: string) {
  const res = await invoke("add_relay", { url });
}

export async function removeRelay(url: string) {
  await invoke("remove_relay", { url });
}

export async function restoreKeyPair(sk: string): Promise<string[]> {
  return await invoke("restore_key_pair", { sk });
}

export async function generateKeyPair(): Promise<string[]> {
  return await invoke("generate_key_pair");
}
