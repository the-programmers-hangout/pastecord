import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import "./index.css"
import "./theme.css"

import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import { NewPasteContent } from './components/NewPasteContent'
import { PasteContent } from './components/PasteContent'

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />
  },
	{
		path: "/:documentId",
		element: <App />
	}
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
		<RouterProvider router={router} />
  </React.StrictMode>,
)
