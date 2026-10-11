import { createHashRouter } from "react-router";
import { Layout } from "../components/Layout";
import { AdditionScreen } from "../screens/AdditionScreen";
import { HomeScreen } from "../screens/HomeScreen";

/** Hash routing: the Tauri WebView serves the front from files, without server-side routes. */
export const router = createHashRouter([
  {
    element: <Layout />,
    children: [
      { index: true, element: <HomeScreen /> },
      { path: "addition", element: <AdditionScreen /> },
    ],
  },
]);
