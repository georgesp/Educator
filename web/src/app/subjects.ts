import type { Subject } from "../api/client";

/** Subjects in the address of a screen: `/exercices/maths`, `/lecons/orthographe`. */
export const SUBJECT_SLUGS: Record<Subject, string> = { Math: "maths", Spelling: "orthographe" };

export const SUBJECTS: readonly Subject[] = ["Math", "Spelling"];

/** The subject of an address segment; mathematics for an unknown one. */
export function subjectOfSlug(slug: string | undefined): Subject {
  return SUBJECTS.find((s) => SUBJECT_SLUGS[s] === slug) ?? "Math";
}
