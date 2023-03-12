import Highlight from "react-highlight";
import { redirect, useLoaderData } from "react-router-dom";

export const PasteContent = ({ content }) => {
  return (
    <div className="mt-4 flex-1 p-0 px-5 bg-transparent font-mono h-100 w-screen border-0 border-0 outline-none scroll-m-1">
      <Highlight>{content}</Highlight>
    </div>
  );
};
