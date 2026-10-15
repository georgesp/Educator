import { createHashRouter, Navigate, type RouteObject } from "react-router";
import { Layout } from "../components/Layout";
import { AvatarScreen } from "../screens/AvatarScreen";
import { BadgesScreen } from "../screens/BadgesScreen";
import { ComponentsDemoScreen } from "../screens/ComponentsDemoScreen";
import { EditProfileScreen } from "../screens/EditProfileScreen";
import { ExamRunScreen, ExamScreen, TimedRunScreen, TimedSetupScreen } from "../screens/ExamScreen";
import { LessonScreen } from "../screens/LessonScreen";
import { LessonsScreen } from "../screens/LessonsScreen";
import { MenuScreen } from "../screens/MenuScreen";
import { ModeScreen } from "../screens/ModeScreen";
import { OptionsScreen } from "../screens/OptionsScreen";
import { PlacementRunScreen, PlacementScreen } from "../screens/PlacementScreen";
import { ParentGateScreen } from "../screens/ParentGateScreen";
import { ParentScreen } from "../screens/ParentScreen";
import { ProfilesScreen } from "../screens/ProfilesScreen";
import { ProgressMapScreen } from "../screens/ProgressMapScreen";
import { ReviewScreen } from "../screens/ReviewScreen";
import { SeriesScreen } from "../screens/SeriesScreen";
import { SkillsScreen } from "../screens/SkillsScreen";
import { SubjectScreen } from "../screens/SubjectScreen";
import { TablesScreen } from "../screens/TablesScreen";

/** Screens of the application, shared by the app and the tests. */
export const screenRoutes: RouteObject[] = [
  { index: true, element: <ProfilesScreen /> },
  { path: "menu", element: <MenuScreen /> },
  { path: "profil/:profileId", element: <EditProfileScreen /> },
  { path: "exercices", element: <SubjectScreen kind="exercises" /> },
  { path: "exercices/:subject", element: <SkillsScreen /> },
  { path: "exercices/:subject/:skillId", element: <ModeScreen /> },
  { path: "serie/:skillId/:mode", element: <SeriesScreen /> },
  { path: "tables", element: <TablesScreen /> },
  { path: "options", element: <OptionsScreen /> },
  { path: "parent", element: <ParentGateScreen /> },
  { path: "parent/espace", element: <ParentScreen /> },
  { path: "examen", element: <ExamScreen /> },
  { path: "examen/:subject", element: <ExamRunScreen /> },
  { path: "evaluation", element: <PlacementScreen /> },
  { path: "evaluation/:subject", element: <PlacementRunScreen /> },
  { path: "defi/:subject", element: <TimedSetupScreen /> },
  { path: "defi/:subject/:seconds", element: <TimedRunScreen /> },
  { path: "lecons", element: <SubjectScreen kind="lessons" /> },
  { path: "lecons/:subject", element: <LessonsScreen /> },
  { path: "lecons/:subject/:lessonId", element: <LessonScreen /> },
  // Lot 3: avatar, badges, progress map, reviews.
  { path: "avatar", element: <AvatarScreen /> },
  { path: "badges", element: <BadgesScreen /> },
  { path: "carte", element: <Navigate to="/carte/maths" replace /> },
  { path: "carte/:subject", element: <ProgressMapScreen /> },
  { path: "revisions/:subject", element: <ReviewScreen /> },
  // Development only: every child component on one page (SUB000105).
  ...(import.meta.env.DEV ? [{ path: "dev/composants", element: <ComponentsDemoScreen /> }] : []),
];

/** Hash routing: the Tauri WebView serves the front from files, without server-side routes. */
export const router = createHashRouter([{ element: <Layout />, children: screenRoutes }]);
