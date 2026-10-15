import { useNavigate, useParams } from "react-router";
import type { Profile } from "../api/client";
import { useApi } from "../app/BackendContext";
import { SUBJECT_SLUGS, subjectOfSlug } from "../app/subjects";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import { SeriesRun, Summary } from "./SeriesScreen";

/**
 * « Mes révisions » (SUB001509): the calculations and the words missed before, due today. The
 * answers move each item in the review queue (the core decides when it comes back).
 */
export function ReviewScreen() {
  return <RequireProfile>{(profile) => <ReviewRun profile={profile} />}</RequireProfile>;
}

function ReviewRun({ profile }: { profile: Profile }) {
  const api = useApi();
  const navigate = useNavigate();
  const subject = subjectOfSlug(useParams().subject);
  const exercises = `/exercices/${SUBJECT_SLUGS[subject]}`;
  return (
    <SeriesRun
      key={subject}
      profile={profile}
      answerMode="Keyboard"
      start={() => api.startReview(profile.id, subject)}
      spokenSummary={(summary) => fr.reviews.finishedSpoken(summary.correctCount, summary.exerciseCount)}
      // Once done, nothing is due any more today: back to the exercises rather than « Recommencer ».
      renderSummary={(summary) => (
        <Summary summary={summary} onAgain={() => void navigate(exercises)} onMenu={() => void navigate("/menu")} againLabel={fr.reviews.backToExercises} />
      )}
    />
  );
}
