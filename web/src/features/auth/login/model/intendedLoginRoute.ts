export function intendedLoginRoute() {
  if (typeof window === "undefined") {
    return "/session";
  }

  const redirect = new URLSearchParams(window.location.search).get("redirect");
  if (redirect?.startsWith("/") && !redirect.startsWith("//")) {
    return redirect;
  }

  return "/session";
}
