import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { useNavigate } from "react-router";
import type { Accessory, AccessorySlot, Profile, WornAccessory } from "../api/client";
import { useApi } from "../app/BackendContext";
import { queryKeys, useCharacters } from "../app/queries";
import { AvatarImage } from "../components/child/AvatarImage";
import { CharacterImage } from "../components/child/CharacterImage";
import { LockIcon } from "../components/icons";
import { RequireProfile } from "../components/RequireProfile";
import { fr } from "../i18n/fr";

const SLOTS: readonly AccessorySlot[] = ["Head", "Eyes", "Neck", "Companion"];

/**
 * My avatar (SUB001505): the avatar in large, the accessories by slot (one per slot), the locked
 * ones with a padlock and their condition, the character always changeable. Every click is saved at
 * once: no « Valider ». Everything works with the mouse alone.
 */
export function AvatarScreen() {
  return <RequireProfile>{(profile) => <AvatarEditor key={profile.id} profile={profile} />}</RequireProfile>;
}

/** The outfit with `accessory` in its slot (or nothing in that slot when `accessory` is null). */
function withSlot(worn: readonly WornAccessory[], all: readonly Accessory[], slot: AccessorySlot, item: WornAccessory | null): WornAccessory[] {
  const others = worn.filter((w) => all.find((a) => a.id === w.accessoryId)?.slot !== slot);
  return item ? [...others, item] : others;
}

function AvatarEditor({ profile }: { profile: Profile }) {
  const api = useApi();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const characters = useCharacters();
  const accessories = useQuery({ queryKey: queryKeys.accessories(profile.id), queryFn: () => api.getAccessories(profile.id), staleTime: 0 });
  const [slot, setSlot] = useState<AccessorySlot>("Head");
  const [tab, setTab] = useState<"accessories" | "characters">("accessories");
  // Shown at once; the core saves it, then the profile list is refreshed.
  const [outfit, setOutfit] = useState({ characterId: profile.characterId, accessories: profile.accessories });
  const [locked, setLocked] = useState<Accessory | null>(null);

  const save = useMutation({
    mutationFn: (next: { characterId: string; accessories: WornAccessory[] }) =>
      api.updateAvatar(profile.id, { characterId: next.characterId, accessories: next.accessories }),
    onSuccess: (saved) => {
      queryClient.setQueryData<Profile[]>(queryKeys.profiles, (list) => list?.map((p) => (p.id === saved.id ? saved : p)));
    },
    onError: () => setOutfit({ characterId: profile.characterId, accessories: profile.accessories }),
  });
  const change = (next: { characterId: string; accessories: WornAccessory[] }) => {
    setOutfit(next);
    save.mutate(next);
  };

  const all = accessories.data ?? [];
  const inSlot = all.filter((a) => a.slot === slot);
  const worn = (id: string) => outfit.accessories.find((w) => w.accessoryId === id);
  const current = inSlot.find((a) => worn(a.id));

  const choose = (accessory: Accessory) => {
    if (!accessory.unlocked) {
      setLocked(accessory);
      return;
    }
    setLocked(null);
    const already = worn(accessory.id);
    // A second click takes it off.
    const item = already ? null : { accessoryId: accessory.id, colorId: accessory.colors[0]?.id ?? "" };
    change({ ...outfit, accessories: withSlot(outfit.accessories, all, slot, item) });
  };

  return (
    <main className="screen avatar-screen">
      <header className="screen-header">
        <button type="button" className="back-button" onClick={() => void navigate("/menu")}>
          ← {fr.series.menu}
        </button>
        <h1 className="title">{fr.avatar.title}</h1>
      </header>

      <div className="avatar-layout">
        <div className="avatar-preview">
          <AvatarImage characterId={outfit.characterId} accessories={outfit.accessories} size={256} alt={fr.avatar.preview(profile.name)} />
          <p className="parent-hint" role="status">
            {save.isPending ? fr.avatar.saving : save.isError ? fr.app.error : fr.avatar.saved}
          </p>
        </div>

        <div className="avatar-choices">
          <div className="avatar-tabs" role="tablist" aria-label={fr.avatar.title}>
            <button type="button" role="tab" aria-selected={tab === "accessories"} className="avatar-tab" onClick={() => setTab("accessories")}>
              {fr.avatar.accessories}
            </button>
            <button type="button" role="tab" aria-selected={tab === "characters"} className="avatar-tab" onClick={() => setTab("characters")}>
              {fr.avatar.character}
            </button>
          </div>

          {tab === "characters" && (
            <ul className="plain-list avatar-grid" aria-label={fr.avatar.character}>
              {characters.data?.map((character) => (
                <li key={character.id}>
                  <button
                    type="button"
                    className="avatar-choice"
                    aria-pressed={outfit.characterId === character.id}
                    aria-label={character.name}
                    onClick={() => change({ ...outfit, characterId: character.id })}
                  >
                    <CharacterImage id={character.id} size={72} />
                  </button>
                </li>
              ))}
            </ul>
          )}

          {tab === "accessories" && (
            <>
              <div className="avatar-slots" role="group" aria-label={fr.avatar.slots}>
                {SLOTS.map((s) => (
                  <button key={s} type="button" className="family-filter-button" aria-pressed={slot === s} onClick={() => setSlot(s)}>
                    {fr.avatar.slotNames[s]}
                  </button>
                ))}
              </div>
              {accessories.isLoading && <p className="message">{fr.app.loading}</p>}
              <ul className="plain-list avatar-grid" aria-label={fr.avatar.slotNames[slot]}>
                <li>
                  <button
                    type="button"
                    className="avatar-choice avatar-choice-none"
                    aria-pressed={!current}
                    onClick={() => change({ ...outfit, accessories: withSlot(outfit.accessories, all, slot, null) })}
                  >
                    {fr.avatar.nothing}
                  </button>
                </li>
                {inSlot.map((accessory) => (
                  <li key={accessory.id}>
                    <button
                      type="button"
                      className={accessory.unlocked ? "avatar-choice" : "avatar-choice avatar-choice-locked"}
                      aria-pressed={worn(accessory.id) !== undefined}
                      aria-disabled={!accessory.unlocked || undefined}
                      aria-label={accessory.unlocked ? accessory.name : `${accessory.name} : ${conditionOf(accessory)}`}
                      onClick={() => choose(accessory)}
                    >
                      <AvatarImage
                        characterId={outfit.characterId}
                        accessories={[{ accessoryId: accessory.id, colorId: worn(accessory.id)?.colorId ?? accessory.colors[0]?.id ?? "" }]}
                        size={72}
                      />
                      <span className="avatar-choice-name">{accessory.name}</span>
                      {!accessory.unlocked && (
                        <span className="avatar-lock" aria-hidden="true">
                          <LockIcon size={20} />
                        </span>
                      )}
                    </button>
                  </li>
                ))}
              </ul>
              {locked && (
                <p className="notice avatar-locked-note" role="status">
                  <LockIcon size={20} /> {fr.avatar.lockedNote(locked.name, conditionOf(locked))}
                </p>
              )}
              {current && current.colors.length > 1 && (
                <div className="avatar-colors" role="group" aria-label={fr.avatar.colors(current.name)}>
                  {current.colors.map((color) => (
                    <button
                      key={color.id}
                      type="button"
                      className="avatar-color"
                      style={{ background: color.main }}
                      aria-label={color.name}
                      aria-pressed={worn(current.id)?.colorId === color.id}
                      onClick={() =>
                        change({ ...outfit, accessories: withSlot(outfit.accessories, all, slot, { accessoryId: current.id, colorId: color.id }) })
                      }
                    />
                  ))}
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </main>
  );
}

/** « Atteins le niveau 5 », « Gagne le badge 7 jours d'entraînement ». */
function conditionOf(accessory: Accessory): string {
  if (accessory.unlock.level !== null) {
    return fr.avatar.reachLevel(accessory.unlock.level);
  }
  return fr.avatar.winBadge(accessory.unlock.badgeTitle ?? "");
}
