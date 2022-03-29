import Button from "./Button";
import React, { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useForm } from "react-hook-form";

export default function ConfigPage() {
  const [contactsState, setContactsState] = useState([])
  const [relaysState, setRelaysState] = useState([])
  const [pkState, setPkState] = useState("")

  const contactForm = useForm();
  const relayForm = useForm();
  const restoreKeyForm = useForm();


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

  const restoreKey = async (data) => {
    try {
      const res = await invoke('restore_key_pair', data)
      setPkState(res);
    } catch (error) {
      console.log(error)
    }
  }
  const generateKeyPair = async () => {
    try {
      const res = await invoke('generate_key_pair')
      restoreKeyForm.setValue("sk", res[0])
      setPkState(res[1]);

    } catch (error) {
      console.log(error)
    }
  }
  const addContact = async (data) => {
    try {
      const res = await invoke('add_contact', data)
      contactForm.reset()
      await getConfig()
    } catch (error) {
      console.log(error)
    }
  }

  const removeContact = async (contact) => {
    try {
      console.log('herer')
      await invoke('remove_contact', { contact });
      await getConfig()
    } catch (error) {
      console.log(error)
    }
  }
  const addRelay = async (data) => {
    try {
      console.log(data)
      const res = await invoke('add_relay', data)
      relayForm.reset()
      await getConfig()
    } catch (error) {
      console.log(error)
    }
  }
  const removeRelay = async (url) => {
    try {
      console.log('herer')
      await invoke('remove_relay', { url });
      await getConfig()
    } catch (error) {
      console.log(error)
    }
  }


  const renderContacts = () => {
    return contactsState.map(contact => {
      return <div key={contact.pk} >

        <li >{contact.alias} - {contact.pk}</li>
        <Button onClick={() => removeContact(contact)}>Remove</Button>
      </div>
    })
  }

  const renderRelays = () => {
    return relaysState.map(relay => {
      return <div key={relay} >

        <li >{relay}</li>
        <Button onClick={() => removeRelay(relay)}>Remove</Button>
      </div>
    })
  }
  return (
    <div className="flex-1 flex flex-col bg-black h-screen">
      <div className="flex flex-col flex-1">
        <div className="bg-gray-1 rounded p-2 m-4">
          <h1 className="text-2xl font-mono font-bold text-center">Config</h1>
          <form onSubmit={restoreKeyForm.handleSubmit(restoreKey)} className="flex p-2">
            <input
              className="flex-1 mr-1 "
              placeholder="!!! Secret key !!!"
              {...restoreKeyForm.register("sk")}
            />

            <Button submit>Restore</Button>
            <Button onClick={() => generateKeyPair()}>Generate</Button>

          </form>
          <>
            {pkState ?
              <input
                className="flex p-2 mr-1 "
                placeholder="Put you SK"
                disabled
                value={pkState}
              /> : null}
          </>

          <h2 className="font-mono text-sm text-center break-all">
            Relays
          </h2>
          <form onSubmit={relayForm.handleSubmit(addRelay)} className="flex p-2">
            <input
              className="flex-1 mr-1 "
              placeholder="New relay URL. Ex: wss://test.com"
              {...relayForm.register("url")}
            />

            <Button submit>Add</Button>
          </form>
          <div>{renderRelays()}</div>

          <h2 className="font-mono text-sm text-center break-all">
            Contacts
          </h2>
          <form onSubmit={contactForm.handleSubmit(addContact)} className="flex p-2">
            <input
              className="flex-1 mr-1"
              placeholder="User alias"
              {...contactForm.register("alias")}
            />
            <input
              className="flex-1 mr-1"
              placeholder="User .....PK"

              {...contactForm.register("pk")}
            />

            <Button submit>Add</Button>
          </form>

          <h2 className="font-mono text-sm text-center break-all">
            Your keys
          </h2>

          <div>
            {renderContacts()}
          </div>
        </div>
      </div>
    </div>
  );
}
