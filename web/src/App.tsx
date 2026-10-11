import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider } from "react-router";
import { BackendGate } from "./app/BackendGate";
import { ProfileProvider } from "./app/ProfileContext";
import { router } from "./app/routes";

const queryClient = new QueryClient({
  defaultOptions: {
    // Local backend: no network flakiness to hide, errors are shown at once.
    queries: { retry: false, refetchOnWindowFocus: false },
  },
});

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BackendGate>
        <ProfileProvider>
          <RouterProvider router={router} />
        </ProfileProvider>
      </BackendGate>
    </QueryClientProvider>
  );
}
