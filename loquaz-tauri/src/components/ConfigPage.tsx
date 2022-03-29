import Button from "./Button";
import React, { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export default function ConfigPage() {
  const [contactsState, setContactsState] = useState([])
  const [relaysState, setRelaysState] = useState([])
  const [newContactAlias, setNewContactAlias] = useState("")
  const [newContactPk, setNewContactPk] = useState("")
  const [newRelayUrl, setNewRelayUrl] = useState("")

  const getConfig = async () => {
    try {
      const [relays, contacts] = await invoke('get_config');
      setContactsState(contacts)
      setRelaysState(relays)
    } catch (error) {
        console.log(error)
    }
  }

  useEffect(() => {
    getConfig().catch(console.log)
  }, []);



  const addContact = async (e: FormEvent) => {
    e.preventDefault()
    try {
      const res = await invoke('add_contact', { alias: newContactAlias, pk: newContactPk })
     await  getConfig()
    } catch (error) {
      console.log(error)
    }
  }

  const removeContact = async (contact) => {
    try {
      console.log('herer')
      await invoke('remove_contact', {contact}); 
     await  getConfig()
    } catch (error) {
      console.log(error)
    }
  }
  const addRelay = async (e: FormEvent) => {
    e.preventDefault()
    try {
      const res = await invoke('add_relay', { url: newRelayUrl })
     await  getConfig()
    } catch (error) {
      console.log(error)
    }
  }
const removeRelay = async (url) => {
    try {
      console.log('herer')
      await invoke('remove_relay', {url}); 
     await  getConfig()
    } catch (error) {
      console.log(error)
    }
  }
  

  const renderContacts = () => {
    return contactsState.map(contact => {
      return <div   key={contact.pk} >
    
      <li >{contact.alias} - {contact.pk}</li>
      <Button  onClick={() => removeContact(contact)}>Remove</Button>
      </div>
    })
  }

  const renderRelays = () => {
    return relaysState.map(relay => {
      return <div   key={relay} >
    
      <li >{relay}</li>
      <Button  onClick={() => removeRelay(relay)}>Remove</Button>
      </div>
    })
  }
  return (
    <div className="flex-1 flex flex-col bg-black h-screen">
      <div className="flex flex-col flex-1">
        <div className="bg-gray-1 rounded p-2 m-4">
          <h1 className="text-2xl font-mono font-bold text-center">Config</h1>
          <h2 className="font-mono text-sm text-center break-all">
            Relays
          </h2>
          <form onSubmit={addRelay} className="flex p-2">
            <input
              className="flex-1 mr-1 "
              value={newRelayUrl}
              placeholder="New relay URL. Ex: wss://test.com"
              onChange={(e) => setNewRelayUrl(e.target.value)}
            />

            <Button submit>Add</Button>
          </form>
<div>{renderRelays()}</div>

          <h2 className="font-mono text-sm text-center break-all">
            Contacts
          </h2>
          <form onSubmit={addContact} className="flex p-2">
            <input
              className="flex-1 mr-1"
              value={newContactAlias}
              placeholder="User alias"
              onChange={(e) => setNewContactAlias(e.target.value)}
            />
            <input
              className="flex-1 mr-1"
              value={newContactPk}
              placeholder="User PK"
              onChange={(e) => setNewContactPk(e.target.value)}
            />

            <Button submit>Add</Button>
          </form>
          <div>
            {renderContacts()}
          </div>
        </div>
      </div>
    </div>
  );
}
