import { useQuery } from "@tanstack/react-query";
import { useApi } from "../../app/BackendContext";
import { queryKeys } from "../../app/queries";

/** A profile character (SVG served by the host), shown as an image. */
export function CharacterImage({ id, size, alt = "" }: { id: string; size: number; alt?: string }) {
  const api = useApi();
  const { data } = useQuery({ queryKey: queryKeys.characterSvg(id), queryFn: () => api.getCharacterSvg(id), staleTime: Infinity });

  return data ? (
    <img className="character" src={`data:image/svg+xml;charset=utf-8,${encodeURIComponent(data)}`} width={size} height={size} alt={alt} />
  ) : (
    <span className="character character-placeholder" style={{ width: size, height: size }} role={alt ? "img" : undefined} aria-label={alt || undefined} />
  );
}
