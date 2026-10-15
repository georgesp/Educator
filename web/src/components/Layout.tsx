import { Outlet } from "react-router";
import { ProfileSettingsEffect } from "../app/ProfileSettingsEffect";
import { SoundToggle } from "./SoundToggle";
import { StatusBar } from "./StatusBar";

export function Layout() {
  return (
    <div className="layout">
      <ProfileSettingsEffect />
      <SoundToggle />
      <Outlet />
      <StatusBar />
    </div>
  );
}
