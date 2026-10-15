import { useQuery } from "@tanstack/react-query";
import type { Profile, WornAccessory } from "../../api/client";
import { useApi } from "../../app/BackendContext";
import { queryKeys } from "../../app/queries";

type AvatarImageProps = {
  characterId: string;
  accessories: readonly WornAccessory[];
  size: number;
  alt?: string;
};

/**
 * An avatar (SUB001504): the character and the accessories it wears, composed as one SVG by the core.
 * Vector drawing: the same at 48 px and at 256 px.
 */
export function AvatarImage({ characterId, accessories, size, alt = "" }: AvatarImageProps) {
  const api = useApi();
  const { data } = useQuery({
    queryKey: queryKeys.avatarSvg(characterId, accessories),
    queryFn: () => api.getAvatarSvg(characterId, [...accessories]),
    staleTime: Infinity,
  });

  return data ? (
    <img className="character avatar" src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(data)}`} width={size} height={size} alt={alt} />
  ) : (
    <span className="character character-placeholder" style={{ width: size, height: size }} role={alt ? "img" : undefined} aria-label={alt || undefined} />
  );
}

/** The avatar of a profile. */
export function ProfileAvatar({ profile, size, alt }: { profile: Profile; size: number; alt?: string }) {
  return <AvatarImage characterId={profile.characterId} accessories={profile.accessories} size={size} alt={alt} />;
}
