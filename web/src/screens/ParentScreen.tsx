import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Fragment, useState } from "react";
import { Navigate, useLocation, useNavigate } from "react-router";
import { ApiError, type Profile, type ProfileStatistics, type SkillStatistics } from "../api/client";
import { useApi } from "../app/BackendContext";
import { isIpad } from "../app/device";
import { isParentSpaceUnlocked, lockParentSpace, parentOrigin } from "../app/parentAccess";
import { useProfileSession } from "../app/ProfileContext";
import { queryKeys, useParentProfile, useProfiles } from "../app/queries";
import { ProfileAvatar } from "../components/child/AvatarImage";
import { CharacterImage } from "../components/child/CharacterImage";
import { DiceIcon, DownloadIcon, TrashIcon, UploadIcon } from "../components/icons";
import { useUpdateSettings } from "../app/ProfileSettingsEffect";
import { useProfileSettings, useSettingsOptions } from "../app/queries";
import { Choice } from "./OptionsScreen";
import { BarChart, LineChart, Sparkline } from "../components/parent/Charts";
import { fr } from "../i18n/fr";

/**
 * Parents' space (SUB001510, SUB001511): the progress of each child, computed by the core from the
 * recorded sessions, and the deletion of a profile. Reached through the « adult » gate only.
 */
export function ParentScreen() {
  if (!isParentSpaceUnlocked()) {
    return <Navigate to="/parent" replace />;
  }

  return <ParentSpace />;
}

function ParentSpace() {
  const navigate = useNavigate();
  const profiles = useProfiles();
  const { profileId } = useProfileSession();
  const [chosen, setChosen] = useState<string | null>(null);
  const list = profiles.data ?? [];
  const selected = list.find((p) => p.id === chosen) ?? list.find((p) => p.id === profileId) ?? list[0];

  // Opened from the parents' card: back to the profile choice; from the options: back to the child's menu.
  const fromProfiles = parentOrigin(useLocation().state) === "/";
  const leave = () => {
    lockParentSpace();
    void navigate(fromProfiles ? "/" : "/menu");
  };

  return (
    <main className="screen parent-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={leave}>
          ← {fromProfiles ? fr.parent.backToProfiles : fr.parent.backToChild}
        </button>
        <h1 className="title">{fr.parent.title}</h1>
      </header>

      {profiles.isSuccess && list.length === 0 && <p className="notice">{fr.parent.noProfiles}</p>}

      {list.length > 0 && (
        <div className="parent-children" role="tablist" aria-label={fr.parent.children}>
          {list.map((profile) => (
            <button
              key={profile.id}
              type="button"
              role="tab"
              id={`child-tab-${profile.id}`}
              aria-selected={profile.id === selected?.id}
              aria-controls="child-panel"
              className="parent-child-tab"
              onClick={() => setChosen(profile.id)}
            >
              <ProfileAvatar profile={profile} size={40} />
              {fr.parent.childTab(profile.name, fr.grades[profile.grade])}
            </button>
          ))}
        </div>
      )}

      {selected && (
        <section id="child-panel" role="tabpanel" aria-labelledby={`child-tab-${selected.id}`} className="parent-panel">
          <ChildSettings key={`settings-${selected.id}`} profile={selected} />
          <ChildStatistics key={selected.id} profile={selected} />
        </section>
      )}

      {list.length > 0 && <ManageProfiles profiles={list} onDeleted={(id) => chosen === id && setChosen(null)} />}

      <ParentCardEditor />

      <BackupSection />
    </main>
  );
}

function ChildStatistics({ profile }: { profile: Profile }) {
  const api = useApi();
  const statistics = useQuery({ queryKey: queryKeys.statistics(profile.id), queryFn: () => api.getProfileStatistics(profile.id), staleTime: 0 });

  if (statistics.isLoading) {
    return <p className="message">{fr.app.loading}</p>;
  }
  if (!statistics.data) {
    return <p className="notice">{fr.parent.loadError}</p>;
  }
  if (statistics.data.summary.sessionCount === 0) {
    return <p className="notice">{fr.parent.empty(profile.name)}</p>;
  }

  return <StatisticsView statistics={statistics.data} />;
}

function StatisticsView({ statistics }: { statistics: ProfileStatistics }) {
  const { summary } = statistics;
  const thisWeek = statistics.weeks[statistics.weeks.length - 1];
  const toReview = statistics.toReview
    .map((id) => statistics.skills.find((s) => s.skillId === id))
    .filter((s): s is SkillStatistics => s !== undefined);
  const lastDay = statistics.days.length - 1;
  const lastWeek = statistics.weeks.length - 1;

  return (
    <>
      <section className="parent-section" aria-labelledby="parent-overview">
        <h2 id="parent-overview" className="section-title">
          {fr.parent.overview}
        </h2>
        <dl className="parent-tiles">
          <Tile label={fr.parent.timeThisWeek} value={fr.parent.duration(thisWeek?.seconds ?? 0)} />
          <Tile label={fr.parent.timeTotal} value={fr.parent.duration(summary.totalSeconds)} />
          <Tile label={fr.parent.pointsEarned} value={String(summary.pointsEarned)} />
          <Tile label={fr.parent.success} value={summary.successRate === null ? fr.parent.none : `${summary.successRate} %`} />
          <Tile label={fr.parent.activeDays} value={fr.parent.activeDaysValue(summary.activeDays, statistics.days.length)} />
          <Tile label={fr.parent.lastActivity} value={summary.lastActivityAt ? fr.parent.instant(summary.lastActivityAt) : fr.parent.none} />
        </dl>

        <div className="parent-review">
          <h3>{fr.parent.toReview}</h3>
          {toReview.length === 0 ? (
            <p>{fr.parent.noToReview}</p>
          ) : (
            <>
              <p className="parent-hint">{fr.parent.toReviewHint}</p>
              <ul className="plain-list">
                {toReview.map((skill) => (
                  <li key={skill.skillId}>
                    <strong>{skill.title}</strong> · {fr.parent.masteryValue(skill.mastery)} · <StateBadge skill={skill} />
                  </li>
                ))}
              </ul>
            </>
          )}
        </div>

        <div className="parent-charts">
          <BarChart
            title={fr.parent.timeByDay}
            caption={fr.parent.timeByDayCaption(statistics.days.length)}
            bars={statistics.days.map((day, i) => ({
              key: day.date,
              label: fr.parent.dayOfMonth(day.date),
              value: day.seconds,
              valueLabel: fr.parent.minutes(day.seconds),
              description: `${fr.parent.day(day.date)} : ${fr.parent.duration(day.seconds)}`,
              current: i === lastDay,
            }))}
          />
          <BarChart
            title={fr.parent.successByWeek}
            caption={fr.parent.successByWeekCaption(statistics.weeks.length)}
            max={100}
            bars={statistics.weeks.map((week, i) => ({
              key: week.start,
              label: fr.parent.shortDay(week.start),
              value: week.successRate ?? 0,
              valueLabel: week.successRate === null ? fr.parent.none : `${week.successRate} %`,
              description: `${fr.parent.week(week.start)} : ${week.successRate === null ? fr.parent.none : `${week.successRate} %`}`,
              current: i === lastWeek,
            }))}
          />
        </div>
      </section>

      <SkillsTable skills={statistics.skills} />

      <section className="parent-section" aria-labelledby="parent-tests">
        <h2 id="parent-tests" className="section-title">
          {fr.parent.tests}
        </h2>
        {statistics.tests.length === 0 ? (
          <p>{fr.parent.noTests}</p>
        ) : (
          <div className="parent-table-scroll">
            <table className="parent-table">
              <thead>
                <tr>
                  <th scope="col">{fr.parent.testColumns.date}</th>
                  <th scope="col">{fr.parent.testColumns.kind}</th>
                  <th scope="col">{fr.parent.testColumns.score}</th>
                  <th scope="col">{fr.parent.testColumns.points}</th>
                </tr>
              </thead>
              <tbody>
                {statistics.tests.map((test) => (
                  <tr key={test.startedAt}>
                    <td>{fr.parent.day(test.date)}</td>
                    <td>
                      {fr.parent.testKind(test.sessionType, test.subject, test.timeLimitSeconds)}
                      {test.abandoned && <span className="parent-hint"> · {fr.parent.stopped}</span>}
                    </td>
                    <td>{fr.parent.score(test.correctCount, test.questionCount)}</td>
                    <td>{test.pointsEarned}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </section>

      <section className="parent-section" aria-labelledby="parent-mistakes">
        <h2 id="parent-mistakes" className="section-title">
          {fr.parent.mistakes}
        </h2>
        {statistics.frequentMistakes.length === 0 ? (
          <p>{fr.parent.noMistakes}</p>
        ) : (
          <>
            <p className="parent-hint">{fr.parent.mistakesHint}</p>
            <div className="parent-table-scroll">
              <table className="parent-table">
                <thead>
                  <tr>
                    <th scope="col">{fr.parent.mistakeColumns.question}</th>
                    <th scope="col">{fr.parent.mistakeColumns.expected}</th>
                    <th scope="col">{fr.parent.mistakeColumns.given}</th>
                    <th scope="col">{fr.parent.mistakeColumns.count}</th>
                    <th scope="col">{fr.parent.mistakeColumns.exercise}</th>
                  </tr>
                </thead>
                <tbody>
                  {statistics.frequentMistakes.map((mistake) => (
                    <tr key={`${mistake.skillId}|${mistake.prompt}|${mistake.expected}`}>
                      <td className="parent-question">{mistake.prompt}</td>
                      <td className="parent-right">{mistake.expected}</td>
                      <td className="parent-wrong">{mistake.usualAnswer || fr.parent.emptyAnswer}</td>
                      <td>{fr.parent.mistakeCount(mistake.mistakeCount, mistake.attemptCount)}</td>
                      <td>{mistake.skillTitle}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </>
        )}
      </section>
    </>
  );
}

function Tile({ label, value }: { label: string; value: string }) {
  return (
    <div className="parent-tile">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function StateBadge({ skill }: { skill: SkillStatistics }) {
  return <span className={`state-badge state-${skill.state}`}>{fr.parent.states[skill.state]}</span>;
}

const TREND_SIGNS = { Up: "↗", Down: "↘", Stable: "→", Unknown: "" } as const;

type Sort = keyof typeof fr.parent.sorts;

/** Display order of the exercises table (the core sends the most recent first). */
const SORTS: Record<Sort, (a: SkillStatistics, b: SkillStatistics) => number> = {
  recent: () => 0,
  weakest: (a, b) => a.mastery - b.mastery || b.answerCount - a.answerCount,
  most: (a, b) => b.completedCount - a.completedCount || b.answerCount - a.answerCount,
  points: (a, b) => b.pointsEarned - a.pointsEarned,
};

/** One line per exercise: how many times it was done, its points and their evolution, its success. */
function SkillsTable({ skills }: { skills: SkillStatistics[] }) {
  const [sort, setSort] = useState<Sort>("recent");
  const [open, setOpen] = useState<string | null>(null);
  const sorted = [...skills].sort(SORTS[sort]);

  return (
    <section className="parent-section" aria-labelledby="parent-exercises">
      <div className="parent-section-header">
        <h2 id="parent-exercises" className="section-title">
          {fr.parent.exercises}
        </h2>
        <label className="parent-sort">
          {fr.parent.sortBy}
          <select value={sort} onChange={(e) => setSort(e.target.value as Sort)}>
            {(Object.keys(fr.parent.sorts) as Sort[]).map((key) => (
              <option key={key} value={key}>
                {fr.parent.sorts[key]}
              </option>
            ))}
          </select>
        </label>
      </div>
      <p className="parent-hint">{fr.parent.exercisesHint}</p>
      <div className="parent-table-scroll">
        <table className="parent-table parent-skills">
          <thead>
            <tr>
              <th scope="col">{fr.parent.columns.exercise}</th>
              <th scope="col">{fr.parent.columns.done}</th>
              <th scope="col">{fr.parent.columns.points}</th>
              <th scope="col">{fr.parent.columns.evolution}</th>
              <th scope="col">
                {fr.parent.columns.mastery}
                <span className="parent-column-hint">{fr.parent.masteryHint}</span>
              </th>
              <th scope="col">{fr.parent.columns.last}</th>
            </tr>
          </thead>
          <tbody>
            {sorted.map((skill) => {
              const points = skill.history.map((p) => p.pointsEarned);
              const isOpen = open === skill.skillId;
              return (
                <Fragment key={skill.skillId}>
                  <tr>
                    <th scope="row">
                      <span className="parent-skill-title">{skill.title}</span>
                      <span className="parent-hint">
                        {fr.subjects.names[skill.subject]}
                        {skill.grade && ` · ${fr.grades[skill.grade]}`}
                      </span>
                    </th>
                    <td>
                      <span className="parent-count">{fr.parent.doneCount(skill.completedCount)}</span>
                      {skill.abandonedCount > 0 && <span className="parent-hint">{fr.parent.abandoned(skill.abandonedCount)}</span>}
                    </td>
                    <td>
                      <span className="parent-count">{skill.pointsEarned}</span>
                      {skill.averagePoints !== null && <span className="parent-hint">{fr.parent.average(skill.averagePoints)}</span>}
                    </td>
                    <td>
                      {points.length === 0 ? (
                        <span className="parent-hint">{fr.parent.noSeries}</span>
                      ) : (
                        // Its name says what it does, the points of the chart and the trend (the drawing itself is hidden).
                        <button type="button" className="parent-spark-button" aria-expanded={isOpen} onClick={() => setOpen(isOpen ? null : skill.skillId)}>
                          <span className="visually-hidden">
                            {[
                              isOpen ? fr.parent.hideDetails(skill.title) : fr.parent.showDetails(skill.title),
                              fr.parent.sparkline(points),
                              fr.parent.trends[skill.trend],
                            ]
                              .filter(Boolean)
                              .join(" · ")}
                          </span>
                          <span aria-hidden="true">
                            <Sparkline values={points} label={fr.parent.sparkline(points)} />
                            {skill.trend !== "Unknown" && (
                              <span className={`trend trend-${skill.trend}`}>
                                {TREND_SIGNS[skill.trend]} {fr.parent.trends[skill.trend]}
                              </span>
                            )}
                          </span>
                        </button>
                      )}
                    </td>
                    <td>
                      <span className="parent-count">{fr.parent.masteryValue(skill.mastery)}</span>
                      <StateBadge skill={skill} />
                    </td>
                    <td>{fr.parent.instant(skill.lastPracticedAt)}</td>
                  </tr>
                  {isOpen && (
                    <tr className="parent-details">
                      <td colSpan={6}>
                        <LineChart
                          title={fr.parent.detailsTitle(skill.title)}
                          points={skill.history.map((p) => ({
                            key: p.startedAt,
                            value: p.pointsEarned,
                            label: fr.parent.shortDay(p.date),
                            description: fr.parent.seriesPoint(fr.parent.day(p.date), p.pointsEarned, p.correctCount, p.questionCount),
                          }))}
                        />
                        {skill.bestPoints !== null && <p className="parent-hint">{fr.parent.best(skill.bestPoints)}</p>}
                      </td>
                    </tr>
                  )}
                </Fragment>
              );
            })}
          </tbody>
        </table>
      </div>
    </section>
  );
}

/** Deleting a profile (moved here from the options, SUB001510). */
function ManageProfiles({ profiles, onDeleted }: { profiles: Profile[]; onDeleted(id: string): void }) {
  const [deleting, setDeleting] = useState<Profile | null>(null);
  return (
    <section className="parent-section" aria-labelledby="parent-manage">
      <h2 id="parent-manage" className="section-title">
        {fr.parent.manage}
      </h2>
      <ul className="plain-list options-profiles">
        {profiles.map((profile) => (
          <li key={profile.id} className="options-profile">
            <ProfileAvatar profile={profile} size={48} />
            <span className="options-profile-name">
              {profile.name} · {fr.grades[profile.grade]}
            </span>
            <button type="button" className="button button-danger" onClick={() => setDeleting(profile)} aria-label={fr.parent.deleteLabel(profile.name)}>
              <TrashIcon size={18} /> {fr.parent.delete}
            </button>
          </li>
        ))}
      </ul>
      {deleting && (
        <DeleteProfile
          key={deleting.id}
          profile={deleting}
          onClose={(deleted) => {
            setDeleting(null);
            if (deleted) {
              onDeleted(deleting.id);
            }
          }}
        />
      )}
    </section>
  );
}

/**
 * Deleting a profile, in two steps: the question first, then the profile's name to type before the
 * button deletes anything.
 */
function DeleteProfile({ profile, onClose }: { profile: Profile; onClose(deleted: boolean): void }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const { profileId, setProfileId } = useProfileSession();
  const [step, setStep] = useState<1 | 2>(1);
  const [typed, setTyped] = useState("");
  const matches = typed.trim().toLocaleLowerCase("fr") === profile.name.toLocaleLowerCase("fr");

  const remove = useMutation({
    mutationFn: () => api.deleteProfile(profile.id),
    onSuccess: async () => {
      queryClient.removeQueries({ queryKey: queryKeys.statistics(profile.id) });
      await queryClient.invalidateQueries({ queryKey: queryKeys.profiles });
      if (profile.id === profileId) {
        setProfileId(null);
      }
      onClose(true);
    },
  });

  return (
    <div className="quit-confirm options-delete" role="alertdialog" aria-labelledby="delete-question" aria-describedby="delete-warning">
      <p id="delete-question">{fr.parent.deleteQuestion(profile.name)}</p>
      <p id="delete-warning" className="message">
        {fr.parent.deleteWarning}
      </p>
      {step === 1 ? (
        <div className="actions">
          <button type="button" className="button" onClick={() => onClose(false)} autoFocus>
            {fr.parent.keep}
          </button>
          <button type="button" className="button button-danger" onClick={() => setStep(2)}>
            {fr.parent.continue}
          </button>
        </div>
      ) : (
        <form
          className="options-delete-confirm"
          onSubmit={(e) => {
            e.preventDefault();
            if (matches) {
              remove.mutate();
            }
          }}
        >
          <label>
            {fr.parent.typeName(profile.name)}
            <input className="name-input" value={typed} onChange={(e) => setTyped(e.target.value)} autoComplete="off" autoFocus />
          </label>
          <div className="actions">
            <button type="button" className="button" onClick={() => onClose(false)}>
              {fr.parent.keep}
            </button>
            <button type="submit" className="button button-danger" disabled={!matches || remove.isPending}>
              {fr.parent.deleteForever}
            </button>
          </div>
          {remove.isError && <p className="message">{fr.app.error}</p>}
        </form>
      )}
    </div>
  );
}

/** The parents' card of the profile choice: its name and its character, created by default by the core. */
function ParentCardEditor() {
  const api = useApi();
  const queryClient = useQueryClient();
  const parent = useParentProfile();
  const [typed, setTyped] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);
  const name = typed ?? parent.data?.name ?? "";

  const update = useMutation({
    mutationFn: async (changes: { name?: string; reroll?: boolean }) => {
      const characterId = changes.reroll ? (await api.drawCharacter(parent.data?.characterId)).id : undefined;
      return api.updateParentProfile({ name: changes.name, characterId });
    },
    onSuccess: (card, changes) => {
      queryClient.setQueryData(queryKeys.parentProfile, card);
      if (changes.name !== undefined) {
        setTyped(null);
        setSaved(true);
      }
    },
  });
  const error = update.error instanceof ApiError && update.error.status === 400 ? update.error.detail : update.isError ? fr.app.error : null;

  if (!parent.data) {
    return null;
  }

  return (
    <section className="parent-section" aria-labelledby="parent-card">
      <h2 id="parent-card" className="section-title">
        {fr.parent.card}
      </h2>
      <p className="parent-hint">{fr.parent.cardHint}</p>
      <div className="parent-card-editor">
        <div className="parent-card-character">
          <CharacterImage id={parent.data.characterId} size={96} alt={fr.parent.cardCharacter(parent.data.name)} />
          <button type="button" className="button" onClick={() => update.mutate({ reroll: true })} disabled={update.isPending}>
            <DiceIcon size={20} /> {fr.parent.cardReroll}
          </button>
        </div>
        <form
          className="parent-card-name"
          onSubmit={(e) => {
            e.preventDefault();
            update.mutate({ name });
          }}
        >
          <label>
            {fr.parent.cardName}
            <input
              className="name-input"
              value={name}
              maxLength={20}
              autoComplete="off"
              onChange={(e) => {
                setTyped(e.target.value);
                setSaved(false);
              }}
              aria-invalid={error ? true : undefined}
              aria-describedby={error ? "parent-card-error" : undefined}
            />
          </label>
          <button type="submit" className="button button-primary" disabled={update.isPending || name === parent.data.name}>
            {fr.parent.cardSave}
          </button>
          {saved && (
            <p className="parent-hint" role="status">
              {fr.parent.cardSaved}
            </p>
          )}
          {error && (
            <p id="parent-card-error" className="form-error" role="alert">
              {error}
            </p>
          )}
        </form>
      </div>
    </section>
  );
}

/**
 * The settings the parents choose for a child (SUB001510, SUB001512): the goal of the day, the timed
 * challenge and its duration, the advised length of a session.
 */
function ChildSettings({ profile }: { profile: Profile }) {
  const settings = useProfileSettings(profile.id).data;
  const options = useSettingsOptions().data;
  const update = useUpdateSettings(profile.id);
  if (!settings || !options) {
    return null;
  }

  return (
    <section className="parent-section" aria-labelledby="parent-settings">
      <h2 id="parent-settings" className="section-title">
        {fr.parent.settings(profile.name)}
      </h2>
      <Choice
        legend={fr.parent.dailyGoal}
        values={options.goalChoices}
        selected={settings.dailyGoal}
        label={(n) => fr.parent.dailyGoalValue(n)}
        onChoose={(dailyGoal) => update.mutate({ dailyGoal })}
      />
      <label className="option-switch">
        <input type="checkbox" checked={settings.timedEnabled} onChange={() => update.mutate({ timedEnabled: !settings.timedEnabled })} />
        <span>{fr.parent.timedEnabled}</span>
      </label>
      {settings.timedEnabled && (
        <Choice
          legend={fr.parent.timedDuration}
          values={options.timedChoices}
          selected={settings.timedSeconds}
          label={(s) => (s === 0 ? fr.parent.timedChildChooses : fr.exam.duration(s))}
          onChoose={(timedSeconds) => update.mutate({ timedSeconds })}
        />
      )}
      <Choice
        legend={fr.parent.sessionLength}
        values={options.sessionChoices}
        selected={settings.sessionMinutes}
        label={(m) => fr.parent.sessionLengthValue(m)}
        onChoose={(sessionMinutes) => update.mutate({ sessionMinutes })}
      />
      <p className="parent-hint">{fr.parent.sessionLengthHint}</p>
      {update.isError && <p className="form-error">{fr.app.error}</p>}
    </section>
  );
}

/**
 * Backup and restore of every profile (SUB001512), to keep them safe or to move to another computer.
 * A restore replaces the profiles of this computer: asked twice. The iPad has no file dialog yet
 * (SUB023814): the section only says so there.
 */
function BackupSection() {
  if (isIpad()) {
    return (
      <section className="parent-section" aria-labelledby="parent-backup">
        <h2 id="parent-backup" className="section-title">
          {fr.parent.backup}
        </h2>
        <p className="parent-hint">{fr.parent.backupOnIpad}</p>
      </section>
    );
  }
  return <BackupActions />;
}

function BackupActions() {
  const api = useApi();
  const queryClient = useQueryClient();
  const { setProfileId } = useProfileSession();
  const [confirm, setConfirm] = useState(false);
  const exported = useMutation({ mutationFn: () => api.exportBackup() });
  const imported = useMutation({
    mutationFn: () => api.importBackup(),
    onSuccess: async (result) => {
      setConfirm(false);
      if (result) {
        setProfileId(null);
        await queryClient.invalidateQueries();
      }
    },
  });
  const message = (error: unknown) => (error instanceof ApiError && error.status === 400 ? fr.parent.backupBadFile : fr.app.error);

  return (
    <section className="parent-section" aria-labelledby="parent-backup">
      <h2 id="parent-backup" className="section-title">
        {fr.parent.backup}
      </h2>
      <p className="parent-hint">{fr.parent.backupHint}</p>
      <div className="actions actions-start">
        <button type="button" className="button" onClick={() => exported.mutate()} disabled={exported.isPending}>
          <DownloadIcon size={22} /> {fr.parent.backupExport}
        </button>
        <button type="button" className="button" onClick={() => setConfirm(true)} disabled={imported.isPending}>
          <UploadIcon size={22} /> {fr.parent.backupImport}
        </button>
      </div>
      {exported.data && (
        <p className="parent-hint" role="status">
          {fr.parent.backupDone(exported.data.profiles, exported.data.path)}
        </p>
      )}
      {exported.isError && <p className="form-error">{message(exported.error)}</p>}
      {confirm && (
        <div className="quit-confirm" role="alertdialog" aria-labelledby="restore-question">
          <p id="restore-question">{fr.parent.restoreQuestion}</p>
          <p className="message">{fr.parent.restoreWarning}</p>
          <div className="actions">
            <button type="button" className="button" onClick={() => setConfirm(false)} autoFocus>
              {fr.parent.keep}
            </button>
            <button type="button" className="button button-danger" onClick={() => imported.mutate()} disabled={imported.isPending}>
              {fr.parent.restoreChoose}
            </button>
          </div>
        </div>
      )}
      {imported.data && (
        <p className="parent-hint" role="status">
          {fr.parent.restoreDone(imported.data.profiles)}
        </p>
      )}
      {imported.isError && <p className="form-error">{message(imported.error)}</p>}
    </section>
  );
}
