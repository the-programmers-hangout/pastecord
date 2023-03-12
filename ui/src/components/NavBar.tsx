import { PencilSquareIcon } from "@heroicons/react/24/solid";
import React from "react";

type NavBarProps = {
	onSave: () => void;
}

export const NavBar = (props: NavBarProps) => {
  return (
    <nav className="bg-white border-gray-200 px-8 py-2.5 dark:bg-gray-800 justify-between flex">
        <a href="/" className="flex items-center">
          <span className="self-center text-xl font-semibold whitespace-nowrap dark:text-white">Pastecord</span>
        </a>
        <div className="hidden w-full md:block md:w-auto" id="navbar-default">
          <ul className="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
            <li>
              <a
                className="block py-2 pl-3 pr-4 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
								onClick={() => props.onSave()}
              >
                save
              </a>
            </li>
          </ul>
        </div>
    </nav>
  );
};
