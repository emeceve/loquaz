function Item({ active }: { active?: boolean }) {
  return (
    <div
      className={`p-4 font-mono flex space-x-4 text-sm items-center ${
        active ? "bg-black" : ""
      }`}
    >
      <div className="h-8 w-8 rounded-3xl bg-red-1"></div>
      <div>
        <strong>@futurepaul</strong>
        <p>I don't understand</p>
      </div>
    </div>
  );
}
export default function ChatList() {
  return (
    <div className="min-w-[20ch] flex-0 bg-gray-1">
      <ul>
        <Item />
        <Item active />
        <Item />
        <Item />
      </ul>
    </div>
  );
}
