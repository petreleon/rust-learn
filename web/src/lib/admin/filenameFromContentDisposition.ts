export function filenameFromContentDisposition(header: string | null) {
  if (!header) {
    return null;
  }

  const quoted = header.match(/filename="([^"]+)"/i);
  if (quoted?.[1]) {
    return quoted[1];
  }

  const bare = header.match(/filename=([^;]+)/i);
  return bare?.[1]?.trim() || null;
}
