export function codeFromStatus(status: number) {
  if (status === 401) {
    return "unauthorized";
  }
  if (status === 403) {
    return "permission_denied";
  }
  if (status === 404) {
    return "not_found";
  }
  if (status === 409) {
    return "conflict";
  }
  if (status >= 500) {
    return "server_error";
  }
  return "admin_error";
}
