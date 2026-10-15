import { useSearchParams } from "react-router";
import type { SkillCategory } from "../../api/client";
import { fr } from "../../i18n/fr";

/** Color rank (1 to 5) of each family: the theme defines `--family-1` to `--family-5`. */
const RANKS: Record<SkillCategory, number> = {
  Numbers: 1,
  AddSubtract: 2,
  Multiply: 3,
  FractionsHalves: 4,
  Problems: 5,
  LettersAndSounds: 1,
  SpellingRules: 2,
  SilentLetters: 3,
  Agreement: 4,
  WordsToKnow: 5,
};

/** Symbols drawn small by the display font (+, −, ×): shown larger. */
const OPERATORS: readonly SkillCategory[] = ["AddSubtract", "Multiply"];

/** The symbol of a family, as shown (hidden from the screen readers, which read its name). */
function Symbol({ category }: { category: SkillCategory }) {
  return (
    <span aria-hidden="true" className={OPERATORS.includes(category) ? "family-operator" : undefined}>
      {fr.families.symbols[category]}
    </span>
  );
}

/** Order of the families in the legend. */
const ORDER = Object.keys(RANKS) as SkillCategory[];

/** CSS class giving `--family-bg` and `--family-fg` to a family. */
export const familyClass = (category: SkillCategory) => `family-${RANKS[category]}`;

/** The families present in a list, once each, in the order of the legend. */
export function familiesOf(categories: readonly (SkillCategory | null | undefined)[]): SkillCategory[] {
  return ORDER.filter((c) => categories.includes(c));
}

/** Symbol of a family on its color, with its name for the screen readers. */
export function FamilySymbol({ category, className }: { category: SkillCategory; className: string }) {
  return (
    <span className={`${className} ${familyClass(category)}`}>
      <Symbol category={category} />
      <span className="visually-hidden">{fr.families.names[category]}</span>
    </span>
  );
}

/** Name of the address parameter holding the family shown (`/exercices/maths?famille=Numbers`). */
const FILTER_PARAM = "famille";

/**
 * The family the child chose to show, kept in the address so that going back to the list keeps it;
 * `null`: every family. A family absent from `categories` (old address) counts as `null`.
 */
export function useFamilyFilter(
  categories: readonly (SkillCategory | null | undefined)[],
): [SkillCategory | null, (category: SkillCategory | null) => void] {
  const [search, setSearch] = useSearchParams();
  const value = search.get(FILTER_PARAM);
  const selected = ORDER.find((c) => c === value && categories.includes(c)) ?? null;
  const select = (category: SkillCategory | null) =>
    setSearch(category === null ? {} : { [FILTER_PARAM]: category }, { replace: true });
  return [selected, select];
}

/**
 * Under the title of the exercises and lessons: « Tout » and one button per family shown (symbol and
 * name), which keeps only that family; pressed again, every family comes back. Hidden with one family.
 */
export function FamilyFilter({
  categories,
  selected,
  onSelect,
}: {
  categories: readonly (SkillCategory | null | undefined)[];
  selected: SkillCategory | null;
  onSelect: (category: SkillCategory | null) => void;
}) {
  const families = familiesOf(categories);
  if (families.length < 2) {
    return null;
  }
  return (
    <div className="family-filter" role="group" aria-label={fr.families.filter}>
      <button type="button" className="family-filter-button family-filter-all" aria-pressed={selected === null} onClick={() => onSelect(null)}>
        {fr.families.all}
      </button>
      {families.map((category) => (
        <button
          key={category}
          type="button"
          className="family-filter-button"
          aria-pressed={selected === category}
          onClick={() => onSelect(selected === category ? null : category)}
        >
          <span className={`family-symbol ${familyClass(category)}`}>
            <Symbol category={category} />
          </span>
          {fr.families.names[category]}
        </button>
      ))}
    </div>
  );
}
