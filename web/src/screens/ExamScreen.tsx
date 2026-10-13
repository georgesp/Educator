import { useQuery } from "@tanstack/react-query";
import { motion } from "motion/react";
import { useNavigate, useParams } from "react-router";
import type { Profile, SeriesSummary, SkillResult, Subject } from "../api/client";
import { useApi } from "../app/BackendContext";
import { SUBJECT_SLUGS, SUBJECTS, subjectOfSlug } from "../app/subjects";
import { BigButton } from "../components/child/BigButton";
import { Fireworks } from "../components/child/Fireworks";
import { Mascot } from "../components/child/Mascot";
import { SpeakButton } from "../components/child/SpeakButton";
import { BookIcon, FlagIcon, PencilIcon, TimerIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";
import { SeriesRun } from "./SeriesScreen";

/** Examen (SUB001414, SUB001415): the review of a subject, or the timed challenge. */
export function ExamScreen() {
  return <RequireProfile>{() => <ExamChoice />}</RequireProfile>;
}

function ExamChoice() {
  const navigate = useNavigate();
  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.exam.title}</h1>
        <SpeakButton text={fr.exam.spoken} autoPlay />
      </header>
      <div className="exam-choice">
        <section className="mode-choice" aria-labelledby="exam-review">
          <h2 id="exam-review" className="section-title">
            {fr.exam.review} · {fr.exam.reviewHint}
          </h2>
          {SUBJECTS.map((subject, i) => (
            <BigButton
              key={subject}
              label={fr.exam.examOf[subject]}
              icon={<FlagIcon />}
              variant="primary"
              autoFocus={i === 0}
              onClick={() => void navigate(`/examen/${SUBJECT_SLUGS[subject]}`)}
            />
          ))}
        </section>
        <section className="mode-choice" aria-labelledby="exam-timed">
          <h2 id="exam-timed" className="section-title">
            {fr.exam.timed} · {fr.exam.timedHint}
          </h2>
          {SUBJECTS.map((subject) => (
            <BigButton key={subject} label={fr.exam.timedOf[subject]} icon={<TimerIcon />} onClick={() => void navigate(`/defi/${SUBJECT_SLUGS[subject]}`)} />
          ))}
        </section>
      </div>
    </main>
  );
}

/** Le bilan of a subject: 15 questions, then the strong points and what to work on again. */
export function ExamRunScreen() {
  return <RequireProfile>{(profile) => <ExamRun profile={profile} />}</RequireProfile>;
}

function ExamRun({ profile }: { profile: Profile }) {
  const api = useApi();
  const subject = subjectOfSlug(useParams().subject);
  return (
    <SeriesRun
      key={subject}
      profile={profile}
      answerMode="Keyboard"
      start={() => api.startExam(profile.id, subject)}
      spokenSummary={(summary) => fr.exam.finishedSpoken(summary.correctCount, summary.exerciseCount)}
      renderSummary={(summary, again) => <ExamSummary summary={summary} subject={subject} onAgain={again} />}
    />
  );
}

function SkillLine({ result, subject }: { result: SkillResult; subject: Subject }) {
  const navigate = useNavigate();
  const slug = SUBJECT_SLUGS[subject];
  return (
    <li className="exam-skill">
      <span className="exam-skill-title">{result.title}</span>
      <span className="exam-skill-score">{fr.exam.skillScore(result.correctCount, result.questionCount)}</span>
      {!result.strong && (
        <span className="exam-skill-links">
          {result.lessonId && (
            <button type="button" className="button" onClick={() => void navigate(`/lecons/${slug}/${encodeURIComponent(result.lessonId ?? "")}`)}>
              <BookIcon size={18} /> {fr.exam.lesson}
            </button>
          )}
          <button type="button" className="button" onClick={() => void navigate(`/exercices/${slug}/${encodeURIComponent(result.skillId)}`)}>
            <PencilIcon size={18} /> {fr.exam.practice}
          </button>
        </span>
      )}
    </li>
  );
}

/** The end of the exam: score, fireworks when everything is right, strong points, notions to work on. */
export function ExamSummary({ summary, subject, onAgain }: { summary: SeriesSummary; subject: Subject; onAgain(): void }) {
  const navigate = useNavigate();
  const perfect = summary.exerciseCount > 0 && summary.correctCount === summary.exerciseCount;
  const strong = summary.skills.filter((s) => s.strong);
  const rework = summary.skills.filter((s) => !s.strong);
  return (
    <motion.section className="summary" role="status" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }}>
      <h1 className="title">{fr.exam.finished}</h1>
      <Mascot situation={perfect ? "SeriesPerfect" : "ExamEnd"} />
      <p className="summary-score">{fr.series.score(summary.correctCount, summary.exerciseCount)}</p>
      {perfect && <Fireworks label={fr.series.allRight} />}
      {perfect && <p className="summary-perfect">{fr.series.allRight}</p>}
      <p className="summary-points">{fr.series.pointsEarned(summary.pointsEarned)}</p>
      <div className="exam-results">
        <section aria-labelledby="exam-strong">
          <h2 id="exam-strong">{fr.exam.strong}</h2>
          {strong.length === 0 ? (
            <p className="message">{fr.exam.noStrong}</p>
          ) : (
            <ul className="plain-list">
              {strong.map((r) => (
                <SkillLine key={r.skillId} result={r} subject={subject} />
              ))}
            </ul>
          )}
        </section>
        <section aria-labelledby="exam-rework">
          <h2 id="exam-rework">{fr.exam.rework}</h2>
          {rework.length === 0 ? (
            <p className="message">{fr.exam.noRework}</p>
          ) : (
            <ul className="plain-list">
              {rework.map((r) => (
                <SkillLine key={r.skillId} result={r} subject={subject} />
              ))}
            </ul>
          )}
        </section>
      </div>
      <div className="actions">
        <button type="button" className="button" onClick={() => void navigate("/menu")}>
          {fr.series.menu}
        </button>
        <button type="button" className="button button-primary" onClick={onAgain}>
          {fr.series.again}
        </button>
      </div>
    </motion.section>
  );
}

/** Défi chrono → the duration, with the best score of each; closed while no skill is mastered. */
export function TimedSetupScreen() {
  return <RequireProfile>{(profile) => <TimedSetup profile={profile} />}</RequireProfile>;
}

function TimedSetup({ profile }: { profile: Profile }) {
  const api = useApi();
  const navigate = useNavigate();
  const subject = subjectOfSlug(useParams().subject);
  const challenge = useQuery({ queryKey: ["timed", profile.id, subject], queryFn: () => api.getTimedChallenge(profile.id, subject), staleTime: 0 });
  const data = challenge.data;
  return (
    <main className="screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/examen")}>
          ← {fr.common.back}
        </button>
        <h1 className="title">{fr.exam.timedOf[subject]}</h1>
      </header>
      {challenge.isError && <p className="message">{fr.app.error}</p>}
      {challenge.isLoading && <p className="message">{fr.app.loading}</p>}
      {data && data.skillCount === 0 && (
        <div className="mode-choice">
          <p className="notice">{fr.exam.notYet}</p>
          <SpeakButton text={fr.exam.notYet} autoPlay compact />
          <BigButton label={fr.menu.exercises} icon={<PencilIcon />} variant="primary" onClick={() => void navigate(`/exercices/${SUBJECT_SLUGS[subject]}`)} />
        </div>
      )}
      {data && data.skillCount > 0 && (
        <div className="mode-choice">
          <div className="screen-instruction">
            <h2 className="section-title">{fr.exam.chooseDuration}</h2>
            <SpeakButton text={fr.exam.chooseDurationSpoken} autoPlay compact />
          </div>
          <div className="timed-durations">
            {data.durations.map((d, i) => (
              <BigButton
                key={d.seconds}
                label={fr.exam.duration(d.seconds)}
                hint={fr.exam.best(d.best)}
                number={String(i + 1).padStart(2, "0")}
                variant={d.seconds === data.defaultSeconds ? "primary" : "default"}
                autoFocus={d.seconds === data.defaultSeconds}
                onClick={() => void navigate(`/defi/${SUBJECT_SLUGS[subject]}/${d.seconds}`)}
              />
            ))}
          </div>
        </div>
      )}
    </main>
  );
}

/** A timed challenge: questions until the bar is empty, then the score and the record. */
export function TimedRunScreen() {
  return <RequireProfile>{(profile) => <TimedRun profile={profile} />}</RequireProfile>;
}

function TimedRun({ profile }: { profile: Profile }) {
  const api = useApi();
  const params = useParams();
  const subject = subjectOfSlug(params.subject);
  const seconds = Number(params.seconds) || 120;
  return (
    <SeriesRun
      key={`${subject}-${seconds}`}
      profile={profile}
      answerMode="Choice"
      start={() => api.startTimed(profile.id, subject, seconds)}
      spokenSummary={(summary) => fr.exam.timedSpoken(summary.correctCount)}
      renderSummary={(summary, again) => <TimedSummary summary={summary} onAgain={again} />}
    />
  );
}

/** The score of a timed challenge; the record (decided by the commands) is celebrated. */
function TimedSummary({ summary, onAgain }: { summary: SeriesSummary; onAgain(): void }) {
  const navigate = useNavigate();
  const record = summary.newRecord;
  const previous = summary.previousBest;
  return (
    <motion.section className="summary" role="status" initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }}>
      <h1 className="title">{fr.exam.timedFinished}</h1>
      <Mascot situation="TimedEnd" />
      <p className="summary-score">{fr.exam.timedScore(summary.correctCount)}</p>
      {record && <Fireworks label={fr.exam.newRecord} />}
      {record && <p className="new-record">{fr.exam.newRecord}</p>}
      {!record && previous !== null && <p className="message">{fr.exam.best(previous)}</p>}
      <p className="summary-points">{fr.series.pointsEarned(summary.pointsEarned)}</p>
      <div className="actions">
        <button type="button" className="button" onClick={() => void navigate("/menu")}>
          {fr.series.menu}
        </button>
        <button type="button" className="button button-primary" onClick={onAgain} autoFocus>
          {fr.series.again}
        </button>
      </div>
    </motion.section>
  );
}
