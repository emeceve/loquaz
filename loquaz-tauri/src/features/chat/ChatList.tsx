import { useAppDispatch, useAppSelector } from "../../common/hooks";
import { selectContacts } from "../config/configSlice";
import { selectConversation, selectCurrentContact } from "./chatSlice";

function Item({ active, contact }: { active?: boolean; contact: any }) {
  const dispatch = useAppDispatch();
  const clickSelectConversation = () =>
    dispatch(selectConversation(contact.pk));

  return (
    <div
      className={`p-4 font-mono flex space-x-4 text-sm items-center ${
        active ? "bg-black" : ""
      }`}
      onClick={() => clickSelectConversation()}
    >
      <div className="h-8 w-8 rounded-3xl bg-red-1"></div>
      <div>
        <strong>@{contact.alias}</strong>
        <p>I don't understand</p>
      </div>
    </div>
  );
}
export default function ChatList() {
  const contacts = useAppSelector(selectContacts);
  const currentContact = useAppSelector(selectCurrentContact);

  return (
    <div className="min-w-[20ch] flex-0 bg-gray-1">
      <ul>
        {contacts.map((contact) => (
          <Item contact={contact} active={contact.pk == currentContact.pk} />
        ))}
      </ul>
    </div>
  );
}
