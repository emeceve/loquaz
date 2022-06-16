import { MouseEventHandler } from "react";

export default function Button({
  children,
  onClick,
  submit = false,
}: {
  children: React.ReactNode;
  onClick?: (e: React.MouseEvent) => {};
  submit?: boolean;
}) {
  return (
    <div className="bg-white p-1 rounded inline-block">
      <button
        type={submit ? "submit" : "button"}
        className="font-mono py-1 px-2 bg-white border-2 border-black rounded text-black"
        onClick={onClick}
      >
        {children}
      </button>
    </div>
  );
}
