import { createHashRouter, type RouteObject } from "react-router";
import { Layout } from "../components/Layout";
import { ComponentsDemoScreen } from "../screens/ComponentsDemoScreen";
import { EditProfileScreen } from "../screens/EditProfileScreen";
import { MenuScreen } from "../screens/MenuScreen";
import { ModeScreen } from "../screens/ModeScreen";
import { ProfilesScreen } from "../screens/ProfilesScreen";
import { SeriesScreen } from "../screens/SeriesScreen";
import { SkillsScreen } from "../screens/SkillsScreen";

/** Screens of the application (lot 1), shared by the app and the tests. */
export const screenRoutes: RouteObject[] = [
  { index: true, element: <ProfilesScreen /> },
  { path: "menu", element: <MenuScreen /> },
  { path: "profil/:profileId", element: <EditProfileScreen /> },
  { path: "exercices", element: <SkillsScreen /> },
  { path: "exercices/:skillId", element: <ModeScreen /> },
  { path: "serie/:skillId/:mode", element: <SeriesScreen /> },
  // Development only: every child component on one page (SUB000105).
  ...(import.meta.env.DEV ? [{ path: "dev/composants", element: <ComponentsDemoScreen /> }] : []),
];

/** Hash routing: the Tauri WebView serves the front from files, without server-side routes. */
export const router = createHashRouter([{ element: <Layout />, children: screenRoutes }]);
