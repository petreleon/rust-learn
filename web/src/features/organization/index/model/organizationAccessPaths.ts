import { type CurrentSession } from "@/lib/session";

export type OrganizationAccessActionKey = "request_creation" | "request_invite" | "review_session";

export type OrganizationAccessAction = {
  cta: string;
  detail: string;
  href: string;
  key: OrganizationAccessActionKey;
  primary: boolean;
  title: string;
};

export function buildOrganizationAccessActions(session: CurrentSession): OrganizationAccessAction[] {
  const account = `${session.user.name} <${session.user.email}>`;

  return [
    {
      cta: "Draft invite request",
      detail: [
        `Ask an organization owner or operator to add ${account} from their Members page.`,
        "This workspace appears here after membership or scoped permissions are granted.",
      ].join(" "),
      href: organizationInviteRequestHref(session),
      key: "request_invite",
      primary: true,
      title: "Existing organization",
    },
    {
      cta: "Draft creation request",
      detail: [
        "Ask a platform admin to create the organization, assign owner permissions,",
        "and add your account. Organization creation is platform-admin controlled today.",
      ].join(" "),
      href: organizationCreationRequestHref(session),
      key: "request_creation",
      primary: false,
      title: "New organization",
    },
    {
      cta: "Review session",
      detail: "Confirm the email and permission scopes attached to this browser session before requesting access.",
      href: "/session",
      key: "review_session",
      primary: false,
      title: "Current access",
    },
  ];
}

function organizationInviteRequestHref(session: CurrentSession) {
  return mailtoHref({
    body: [
      `Please add ${session.user.name} (${session.user.email}) to your RustLearn organization.`,
      "You can do this from Organizations > Members by adding my account email.",
    ].join("\n\n"),
    subject: "RustLearn organization access request",
  });
}

function organizationCreationRequestHref(session: CurrentSession) {
  return mailtoHref({
    body: [
      `Please create a RustLearn organization and add ${session.user.name} (${session.user.email}).`,
      "After the organization exists, assign owner/member access so it appears in my workspace selector.",
    ].join("\n\n"),
    subject: "RustLearn organization creation request",
  });
}

function mailtoHref({ body, subject }: { body: string; subject: string }) {
  return `mailto:?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
}
