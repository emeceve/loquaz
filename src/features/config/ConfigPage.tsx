import Button from "../../common/components/Button";
import React, { useEffect } from "react";
import { useForm } from "react-hook-form";
import {
  loadConfig,
  addContact,
  selectKeys,
  removeContact,
  removeRelay,
  addRelay,
  restoreKeyPair,
  generateKeyPair,
  resetKeys,
} from "./configSlice";
import { useAppDispatch, useAppSelector } from "../../common/hooks";
import { Contact } from "../../services/config";

export default function ConfigPage() {
  const configState = useAppSelector((state) => state.config);
  const keys = useAppSelector(selectKeys);
  const dispatch = useAppDispatch();

  const contactForm = useForm();
  const relayForm = useForm();
  const restoreKeyForm = useForm({
    defaultValues: {
      sk: keys.sk,
    },
  });

  useEffect(() => {
    dispatch(loadConfig());
  }, []);

  const submitRestoreKey = ({ sk }: { sk: string }) => {
    dispatch(restoreKeyPair(sk));
  };
  const clickGenerateKeyPair = () => {
    dispatch(generateKeyPair());
  };

  const clickResetKeys = () => {
    dispatch(resetKeys());
    restoreKeyForm.reset();
  };

  const submitAddContact = (data: any) => {
    dispatch(addContact(data));
    contactForm.reset();
  };

  const clickRemoveContact = (contact: Contact) => {
    dispatch(removeContact(contact));
  };
  const submitAddRelay = ({ url }: any) => {
    dispatch(addRelay(url));
    relayForm.reset();
  };
  const clickRemoveRelay = async (url: string) => {
    dispatch(removeRelay(url));
  };

  const renderContacts = () => {
    return configState.contacts.map((contact) => {
      return (
        <div key={contact.pk}>
          <li>
            {contact.alias} - {contact.pk}
          </li>
          <Button onClick={() => clickRemoveContact(contact)}>Remove</Button>
        </div>
      );
    });
  };

  const renderRelays = () => {
    return configState.relays.map((relay) => {
      return (
        <div key={relay}>
          <li>{relay}</li>
          <Button onClick={() => clickRemoveRelay(relay)}>Remove</Button>
        </div>
      );
    });
  };
  return (
    <div className="flex-1 flex flex-col bg-black h-screen">
      <div className="flex flex-col flex-1">
        <div className="bg-gray-1 rounded p-2 m-4">
          <h1 className="text-2xl font-mono font-bold text-center">Config</h1>
          <form
            onSubmit={restoreKeyForm.handleSubmit(submitRestoreKey)}
            className="flex p-2"
          >
            {!keys.sk ? (
              <input
                className="flex-1 mr-1 "
                placeholder="!!! Secret key !!!"
                {...restoreKeyForm.register("sk")}
              />
            ) : (
              <input className="flex-1 mr-1 " disabled value={keys.sk} />
            )}

            {keys.sk && keys.pk ? (
              <Button onClick={() => clickResetKeys()}>Reset</Button>
            ) : (
              <>
                <Button submit>Restore</Button>
                <Button onClick={() => clickGenerateKeyPair()}>Generate</Button>
              </>
            )}
          </form>
          <>
            {keys.pk ? (
              <input className="flex p-2 mr-1 " disabled value={keys.pk} />
            ) : null}
          </>

          <h2 className="font-mono text-sm text-center break-all">Relays</h2>
          <form
            onSubmit={relayForm.handleSubmit(submitAddRelay)}
            className="flex p-2"
          >
            <input
              className="flex-1 mr-1 "
              placeholder="New relay URL. Ex: wss://test.com"
              {...relayForm.register("url")}
            />

            <Button submit>Add</Button>
          </form>
          <div>{renderRelays()}</div>

          <h2 className="font-mono text-sm text-center break-all">Contacts</h2>
          <form
            onSubmit={contactForm.handleSubmit(submitAddContact)}
            className="flex p-2"
          >
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

          <h2 className="font-mono text-sm text-center break-all">Your keys</h2>

          <div>{renderContacts()}</div>
        </div>
      </div>
    </div>
  );
}
