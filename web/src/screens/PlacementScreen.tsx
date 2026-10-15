import { motion } from "motion/react";
import { Navigate, useNavigate, useParams } from "react-router";
import type { Profile, SeriesSummary, Subject } from "../api/client";
import { useApi } from "../app/BackendContext";
import { usePlacementStatus } from "../app/queries";
import { SUBJECT_SLUGS, SUBJECTS, subjectOfSlug } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { Mascot } from "../components/child/Mascot";
import { SpeakButton } from "../components/child/SpeakButton";
import { FlagIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import { ExamResults } from "./ExamScreen";
import { SeriesRun } from "./SeriesScreen";

/**
 * Level assessment (SUB001418): offered after the creation of a profile, once per subject. Nothing
 * left to do (all done, or nothing to ask): straight to the menu.
 */
export function PlacementScreen() {
  return <RequireProfile>{(profile) => <PlacementIntro profile={profile} />}</RequireProfile>;
}

function PlacementIntro({ profile }: { profile: Profile }) {
  const navigate = useNavigate();
  const status = usePlacementStatus(profile.id);

  if (status.isLoading) {
    return (
      <main className="centered">
        <p className="message">{fr.app.loading}</p>
      </main>
    );
  }

  const available = SUBJECTS.filter((s) => status.data?.subjects.some((x) => x.subject === s && x.available));
  if (status.isError || available.length === 0) {
    return <Navigate to="/menu" replace />;
  }

  const seconds = status.data?.questionSeconds ?? 20;
  return (
    <main className="screen">
      <header className="screen-header">
        <h1 className="title">{fr.placement.title}</h1>
        <SpeakButton text={fr.placement.spoken(seconds)} />
      </header>
      <div className="placement-intro">
        <p className="notice">{fr.placement.explain(seconds)}</p>
        <p className="message">{fr.placement.onlyOnce}</p>
        <section className="mode-choice" aria-label={fr.placement.title}>
          {available.map((subject, i) => (
            <BigButton
              key={subject}
              label={fr.placement.startOf[subject]}
              icon={<FlagIcon />}
              variant="primary"
              autoFocus={i === 0}
              onClick={() => void navigate(`/evaluation/${SUBJECT_SLUGS[subject]}`)}
            />
          ))}
        </section>
        <div className="actions">
          <button type="button" className="button" onClick={() => void navigate("/menu")}>
            {fr.placement.later}
          </button>
        </div>
      </div>
    </main>
  );
}

/** The assessment of a subject: the questions with a timer, then the review. */
export function PlacementRunScreen() {
  return <RequireProfile>{(profile) => <PlacementRun profile={profile} />}</RequireProfile>;
}

function PlacementRun({ profile }: { profile: Profile }) {
  const api = useApi();
  const subject = subjectOfSlug(useParams().subject);
  return (
    <SeriesRun
      key={subject}
      profile={profile}
      answerMode="Keyboard"
      start={() => api.startPlacement(profile.id, subject)}
      spokenSummary={(summary) => fr.placement.finishedSpoken(summary.correctCount, summary.exerciseCount)}
      renderSummary={(summary) => <PlacementSummary profile={profile} summary={summary} subject={subject} />}
    />
  );
}

/** The end of the assessment: score, strong points, notions to work on, then the other subject if it is still to do. */
function PlacementSummary({ profile, summary, subject }: { profile: Profile; summary: SeriesSummary; subject: Subject }) {
  const navigate = useNavigate();
  // The status is read again on arrival (never fresh): the finished assessment is no longer offered.
  const status = usePlacementStatus(profile.id);
  const next = status.isFetching ? undefined : SUBJECTS.find((s) => s !== subject && status.data?.subjects.some((x) => x.subject === s && x.available));
  return (
    <motion.section className="summary" role="status" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }}>
      <h1 className="title">{fr.placement.finished}</h1>
      <Mascot situation="ExamEnd" />
      <p className="summary-score">{fr.series.score(summary.correctCount, summary.exerciseCount)}</p>
      <p className="message">{fr.placement.thanks}</p>
      <ExamResults summary={summary} subject={subject} />
      <div className="actions">
        <button type="button" className="button" onClick={() => void navigate("/menu")}>
          {fr.series.menu}
        </button>
        {next && (
          <button type="button" className="button button-primary" onClick={() => void navigate(`/evaluation/${SUBJECT_SLUGS[next]}`)} autoFocus>
            {fr.placement.next(fr.placement.startOf[next])}
          </button>
        )}
      </div>
    </motion.section>
  );
}
