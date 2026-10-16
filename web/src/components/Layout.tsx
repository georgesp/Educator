import { Outlet } from "react-router";
import { useApi } from "../app/BackendContext";
import { ProfileSettingsEffect } from "../app/ProfileSettingsEffect";
import { useSyncInterruption } from "./parent/SyncSection";
import { SoundToggle } from "./SoundToggle";
import { StatusBar } from "./StatusBar";

/** Lot tablette 2: whatever the screen, a synchronisation in progress stops when the application goes to the background. */
function SyncInterruption() {
  useSyncInterruption(useApi());
  return null;
}

export function Layout() {
  return (
    <div className="layout">
      <ProfileSettingsEffect />
      <SyncInterruption />
      <SoundToggle />
      <Outlet />
      <StatusBar />
    </div>
  );
}
