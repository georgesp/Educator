import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import { type SyncDevice, type SyncPairing, type SyncResult, type SyncStatus } from "../../api/client";
import type { UiApi } from "../../api/ui";
import { useApi } from "../../app/BackendContext";
import { queryKeys } from "../../app/queries";
import { fr } from "../../i18n/fr";

const t = fr.parent.sync;

/** While shown, the state is read again regularly (a tablet paired, a synchronisation done). */
const REFRESH_MS = 4000;

/**
 * The application goes to the background: an exchange in progress stops after its batch and
 * resumes next time (SUB023811, SUB027602).
 */
export function useSyncInterruption(api: UiApi) {
  useEffect(() => {
    const onChange = () => {
      if (document.visibilityState === "hidden") {
        void api.interruptSync().catch(() => undefined);
      }
    };
    document.addEventListener("visibilitychange", onChange);
    return () => document.removeEventListener("visibilitychange", onChange);
  }, [api]);
}

/**
 * Section « Synchronisation » of the parents' space (SUB027603): on the computer, the reception,
 * the pairing code and the tablets; on the iPad, the pairing, « Synchroniser maintenant » and the
 * questions « Est-ce le même enfant ? ». Everything is decided by the core; this only shows it.
 */
export function SyncSection() {
  const api = useApi();
  const queryClient = useQueryClient();
  const status = useQuery({ queryKey: queryKeys.syncStatus, queryFn: () => api.getSyncStatus(), refetchInterval: REFRESH_MS, retry: false });
  useSyncInterruption(api);

  // The computer listens while this screen is open.
  useEffect(() => {
    void api
      .openSyncScreen()
      .then((s) => queryClient.setQueryData(queryKeys.syncStatus, s))
      .catch(() => undefined);
    return () => {
      void api.closeSyncScreen().catch(() => undefined);
    };
  }, [api, queryClient]);

  if (!status.data) {
    return null;
  }

  return status.data.role === "Computer" ? <ComputerSync status={status.data} /> : <TabletSync status={status.data} />;
}

function useSetStatus() {
  const queryClient = useQueryClient();
  return (status: SyncStatus) => queryClient.setQueryData(queryKeys.syncStatus, status);
}

// ── Computer ────────────────────────────────────────────────────────────────────────────────

function ComputerSync({ status }: { status: SyncStatus }) {
  const api = useApi();
  const setStatus = useSetStatus();
  const [pairing, setPairing] = useState<SyncPairing | null>(null);
  const devicesAtPairing = useRef(0);
  const receive = useMutation({ mutationFn: (enabled: boolean) => api.setSyncReceive(enabled), onSuccess: setStatus });
  const pair = useMutation({
    mutationFn: () => api.startSyncPairing(),
    onSuccess: (offer) => {
      devicesAtPairing.current = status.devices.length;
      setPairing(offer);
    },
  });
  const closePairing = () => {
    setPairing(null);
    void api.cancelSyncPairing().then(setStatus).catch(() => undefined);
  };
  // A tablet just paired: the code is no longer needed.
  const justPaired = pairing !== null && status.devices.length > devicesAtPairing.current;
  useEffect(() => {
    if (justPaired) {
      setPairing(null);
    }
  }, [justPaired]);

  return (
    <section className="parent-section" aria-labelledby="parent-sync">
      <h2 id="parent-sync" className="section-title">
        {t.titleComputer}
      </h2>
      <p className="parent-hint">{t.hint}</p>
      <p role="status" className="sync-state">
        {status.problem ? t.problem : status.receiving ? (status.address ? t.receiving(status.address) : t.receivingNoAddress) : t.notReceiving}
      </p>
      <label className="option-switch">
        <input type="checkbox" checked={status.receiveAlways} onChange={() => receive.mutate(!status.receiveAlways)} disabled={receive.isPending} />
        <span>{t.receiveAlways}</span>
      </label>
      <p className="parent-hint">{t.receiveAlwaysHint}</p>

      {pairing ? (
        <div className="sync-pairing">
          <p>{t.pairHint}</p>
          <div className="sync-qr" role="img" aria-label={t.qrLabel} dangerouslySetInnerHTML={{ __html: pairing.qrSvg }} />
          <p className="parent-hint">
            {t.manual} {t.manualAddress(pairing.address)}, <strong>{t.manualCode(pairing.code)}</strong>
          </p>
          <p className="parent-hint">{t.expires}</p>
          <div className="actions actions-start">
            <button type="button" className="button" onClick={closePairing}>
              {t.closePairing}
            </button>
          </div>
        </div>
      ) : (
        <div className="actions actions-start">
          <button type="button" className="button button-primary" onClick={() => pair.mutate()} disabled={pair.isPending}>
            {t.pair}
          </button>
        </div>
      )}
      {pair.isError && <p className="form-error">{t.problem}</p>}

      <h3>{t.devicesComputer}</h3>
      {status.devices.length === 0 ? (
        <p className="parent-hint">{t.noDevices}</p>
      ) : (
        <ul className="plain-list sync-devices">
          {status.devices.map((device) => (
            <DeviceRow key={device.id} device={device} removeLabel={t.remove} />
          ))}
        </ul>
      )}
    </section>
  );
}

function DeviceRow({ device, removeLabel }: { device: SyncDevice; removeLabel: string }) {
  const api = useApi();
  const setStatus = useSetStatus();
  const [renaming, setRenaming] = useState(false);
  const [name, setName] = useState(device.name);
  const [confirm, setConfirm] = useState(false);
  const rename = useMutation({
    mutationFn: () => api.renameSyncDevice(device.id, name),
    onSuccess: (status) => {
      setStatus(status);
      setRenaming(false);
    },
  });
  const remove = useMutation({ mutationFn: () => api.removeSyncDevice(device.id), onSuccess: setStatus });

  return (
    <li className="sync-device">
      {renaming ? (
        <form
          className="sync-rename"
          onSubmit={(e) => {
            e.preventDefault();
            rename.mutate();
          }}
        >
          <label>
            {t.renameLabel(device.name)}
            <input className="name-input" value={name} maxLength={30} autoComplete="off" autoCorrect="off" autoCapitalize="off" onChange={(e) => setName(e.target.value)} />
          </label>
          <button type="submit" className="button button-primary" disabled={rename.isPending || name.trim() === ""}>
            {t.renameSave}
          </button>
          <button type="button" className="button" onClick={() => setRenaming(false)}>
            {t.renameCancel}
          </button>
        </form>
      ) : (
        <>
          <strong>{device.name}</strong>
          <span className="parent-hint">{device.lastSyncAt ? t.lastSync(device.lastSyncAt) : t.neverSynced}</span>
          <div className="actions actions-start">
            <button type="button" className="button" onClick={() => setRenaming(true)}>
              {t.rename}
            </button>
            <button type="button" className="button button-danger" aria-label={t.removeLabel(device.name)} onClick={() => setConfirm(true)}>
              {removeLabel}
            </button>
          </div>
        </>
      )}
      {confirm && (
        <div className="quit-confirm" role="alertdialog" aria-labelledby={`remove-${device.id}`}>
          <p id={`remove-${device.id}`}>{t.removeQuestion(device.name)}</p>
          <p className="message">{t.removeWarning}</p>
          <div className="actions">
            <button type="button" className="button" onClick={() => setConfirm(false)} autoFocus>
              {t.keepDevice}
            </button>
            <button type="button" className="button button-danger" onClick={() => remove.mutate()} disabled={remove.isPending}>
              {t.removeConfirm}
            </button>
          </div>
        </div>
      )}
    </li>
  );
}

// ── iPad ────────────────────────────────────────────────────────────────────────────────────

function outcomeText(result: SyncResult): string {
  return result.outcome === "Done" ? t.done(result.sent, result.received) : t.outcomes[result.outcome];
}

function TabletSync({ status }: { status: SyncStatus }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const setStatus = useSetStatus();
  const [message, setMessage] = useState<string | null>(null);
  const [manual, setManual] = useState(false);
  const [address, setAddress] = useState("");
  const [code, setCode] = useState("");
  const [confirmForget, setConfirmForget] = useState(false);
  const computer = status.devices[0];

  const afterExchange = async (result: SyncResult) => {
    setStatus(result.status);
    setMessage(outcomeText(result));
    if (result.outcome === "Done") {
      await queryClient.invalidateQueries();
    }
  };
  const sync = useMutation({ mutationFn: () => api.syncNow(false), onSuccess: afterExchange, onError: () => setMessage(t.outcomes.Failed) });
  const pair = useMutation({
    mutationFn: async (request: { link?: string; address?: string; code?: string }) => api.pairWithComputer(request),
    onSuccess: (result) => {
      setStatus(result.status);
      setMessage(result.outcome === "Done" ? t.paired : t.outcomes[result.outcome]);
      if (result.outcome === "Done") {
        setManual(false);
        sync.mutate();
      }
    },
    onError: () => setMessage(t.outcomes.Failed),
  });
  const scan = useMutation({
    mutationFn: () => api.scanPairingCode(),
    onSuccess: (link) => {
      if (link) {
        pair.mutate({ link });
      }
    },
    // Camera refused (or none): the short code instead.
    onError: () => {
      setMessage(t.cameraRefused);
      setManual(true);
    },
  });
  const answer = useMutation({
    mutationFn: ({ local, same }: { local: string; same: boolean }) => api.answerSyncQuestion(local, same),
    onSuccess: (next) => {
      setStatus(next);
      // Every question answered: the synchronisation goes on.
      if (next.questions.length === 0) {
        sync.mutate();
      }
    },
  });
  const forget = useMutation({
    mutationFn: (id: string) => api.removeSyncDevice(id),
    onSuccess: (next) => {
      setStatus(next);
      setConfirmForget(false);
      setMessage(null);
    },
  });
  const busy = sync.isPending || pair.isPending || scan.isPending;

  return (
    <section className="parent-section" aria-labelledby="parent-sync">
      <h2 id="parent-sync" className="section-title">
        {t.titleTablet}
      </h2>
      <p className="parent-hint">{t.hint}</p>

      {status.questions.length > 0 && (
        <div className="sync-questions">
          <h3>{t.questionsTitle}</h3>
          {status.questions.map((q) => (
            <div key={q.localProfileId} className="sync-question" role="group" aria-label={t.question(q.localName)}>
              <p>{t.question(q.localName)}</p>
              <div className="actions actions-start">
                <button type="button" className="button button-primary" onClick={() => answer.mutate({ local: q.localProfileId, same: true })} disabled={answer.isPending}>
                  {t.same}
                </button>
                <button type="button" className="button" onClick={() => answer.mutate({ local: q.localProfileId, same: false })} disabled={answer.isPending}>
                  {t.different}
                </button>
              </div>
              <p className="parent-hint">
                {t.sameHint} {t.differentHint}
              </p>
            </div>
          ))}
        </div>
      )}

      {computer ? (
        <>
          <p>
            <strong>{t.pairedWith(computer.name)}</strong> · {computer.lastSyncAt ? t.lastSync(computer.lastSyncAt) : t.neverSynced}
          </p>
          <div className="actions actions-start">
            <button type="button" className="button button-primary" onClick={() => sync.mutate()} disabled={busy}>
              {sync.isPending ? t.syncing : t.syncNow}
            </button>
            <button type="button" className="button button-danger" onClick={() => setConfirmForget(true)}>
              {t.forget}
            </button>
          </div>
          {confirmForget && (
            <div className="quit-confirm" role="alertdialog" aria-labelledby="forget-question">
              <p id="forget-question">{t.forgetQuestion}</p>
              <p className="message">{t.forgetWarning}</p>
              <div className="actions">
                <button type="button" className="button" onClick={() => setConfirmForget(false)} autoFocus>
                  {t.keepComputer}
                </button>
                <button type="button" className="button button-danger" onClick={() => forget.mutate(computer.id)} disabled={forget.isPending}>
                  {t.forgetConfirm}
                </button>
              </div>
            </div>
          )}
        </>
      ) : (
        <>
          <p>{t.notPaired}</p>
          <p className="parent-hint">{t.scanHint}</p>
          <div className="actions actions-start">
            <button type="button" className="button button-primary" onClick={() => scan.mutate()} disabled={busy}>
              {t.pairWithComputer}
            </button>
            <button type="button" className="button" onClick={() => setManual(true)}>
              {t.noCamera}
            </button>
          </div>
          {manual && (
            <form
              className="sync-manual"
              onSubmit={(e) => {
                e.preventDefault();
                pair.mutate({ address, code });
              }}
            >
              <label>
                {t.addressLabel}
                <input className="name-input" value={address} inputMode="decimal" autoComplete="off" autoCorrect="off" autoCapitalize="off" spellCheck={false} onChange={(e) => setAddress(e.target.value)} />
              </label>
              <label>
                {t.codeLabel}
                <input className="name-input" value={code} autoComplete="off" autoCorrect="off" autoCapitalize="characters" spellCheck={false} onChange={(e) => setCode(e.target.value)} />
              </label>
              <button type="submit" className="button button-primary" disabled={busy || address.trim() === "" || code.trim() === ""}>
                {t.pairSubmit}
              </button>
            </form>
          )}
        </>
      )}

      {message && (
        <p className="parent-hint" role="status">
          {message}
        </p>
      )}
    </section>
  );
}
