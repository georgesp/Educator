import { Outlet } from "react-router";
import { StatusBar } from "./StatusBar";

export function Layout() {
  return (
    <div className="layout">
      <Outlet />
      <StatusBar />
    </div>
  );
}
