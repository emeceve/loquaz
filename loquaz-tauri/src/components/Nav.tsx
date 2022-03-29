import contact from "../assets/pixelarticons_contact.svg";
import sliders from "../assets/pixelarticons_sliders.svg";
import message from "../assets/pixelarticons_message.svg";
import {Link} from "react-router-dom";

export default function Nav() {
  return (
    <nav className="flex-0">
      <ul className="flex flex-col items-center p-4 space-y-4">
        <li>
          <Link to="/"> 
            <img src={message} width={"32px"} height={"32px"} />
          </Link> 
        </li>
        <li>
          <img src={contact} width={"32px"} height={"32px"} />
        </li>
        <li>
          <Link to="/config">
            <img src={sliders} width={"32px"} height={"32px"} />
          </Link>
        </li>
      </ul>
    </nav>
  );
}
