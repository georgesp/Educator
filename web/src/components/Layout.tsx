import { Outlet } from "react-router";
import { SoundToggle } from "./SoundToggle";
import { StatusBar } from "./StatusBar";

export function Layout() {
  return (
    <div className="layout">
      <SoundToggle />
      <Outlet />
      <StatusBar />
    </div>
  );
}
