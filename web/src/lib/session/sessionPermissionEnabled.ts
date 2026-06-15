import { type CurrentSession } from "./CurrentSession";
import { type PlatformSessionScope } from "./PlatformSessionScope";

export type SessionPermissionGroup = {
  label: string;
  permissions: string[];
  scope: "platform" | "organization" | "course";
};

export function sessionScopePermissionEnabled(
  scope: PlatformSessionScope | null | undefined,
  permission: string,
) {
  if (!scope) {
    return false;
  }

  return (
    scope.effective_permissions.includes(permission) &&
    scope.capabilities.some(
      (capability) => capability.enabled && capability.permissions.includes(permission),
    )
  );
}

export function sessionPermissionEnabled(
  session: CurrentSession | null | undefined,
  permission: string,
) {
  if (!session) {
    return false;
  }

  if (permission === "SUBMIT_TEACHER_APPLICATION") {
    return session.access.teacher_application;
  }

  return (
    sessionScopePermissionEnabled(session.platform, permission) ||
    session.organizations.some((organization) =>
      sessionScopePermissionEnabled(organization, permission),
    ) ||
    session.courses.some((course) => sessionScopePermissionEnabled(course, permission))
  );
}

export function sessionCapabilityPermissionGroups(
  session: CurrentSession | null | undefined,
): SessionPermissionGroup[] {
  if (!session) {
    return [
      { label: "Platform", permissions: [], scope: "platform" },
      { label: "Organizations", permissions: [], scope: "organization" },
      { label: "Courses", permissions: [], scope: "course" },
    ];
  }

  return [
    {
      label: "Platform",
      permissions: uniqueSorted([
        ...activeScopePermissions(session.platform),
        ...(session.access.teacher_application ? ["SUBMIT_TEACHER_APPLICATION"] : []),
      ]),
      scope: "platform",
    },
    {
      label: "Organizations",
      permissions: mergedScopePermissions(session.organizations),
      scope: "organization",
    },
    {
      label: "Courses",
      permissions: mergedScopePermissions(session.courses),
      scope: "course",
    },
  ];
}

export function countSessionCapabilityPermissions(
  session: CurrentSession | null | undefined,
) {
  return sessionCapabilityPermissionGroups(session).reduce(
    (total, group) => total + group.permissions.length,
    0,
  );
}

function mergedScopePermissions(scopes: readonly PlatformSessionScope[]) {
  return Array.from(
    scopes.reduce((permissions, scope) => {
      activeScopePermissions(scope).forEach((permission) => permissions.add(permission));
      return permissions;
    }, new Set<string>()),
  ).sort();
}

function uniqueSorted(permissions: string[]) {
  return Array.from(new Set(permissions)).sort();
}

function activeScopePermissions(scope: PlatformSessionScope) {
  const effectivePermissions = new Set(scope.effective_permissions);
  return Array.from(
    scope.capabilities.reduce((permissions, capability) => {
      if (!capability.enabled) {
        return permissions;
      }
      capability.permissions.forEach((permission) => {
        if (effectivePermissions.has(permission)) {
          permissions.add(permission);
        }
      });
      return permissions;
    }, new Set<string>()),
  ).sort();
}
