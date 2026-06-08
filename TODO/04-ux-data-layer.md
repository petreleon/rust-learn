# UX And Data Layer

## Design System And UX Quality

- [ ] Establish reusable layout primitives: app shell, page header, tab set,
      toolbar, card, table, detail panel, form section, modal, toast, and
      status banner.
- [ ] Establish form components with validation, helper text, disabled reasons,
      loading state, optimistic-safe submission, and server error rendering.
- [ ] Establish data components for tables, filters, pagination, empty states,
      skeletons, and refresh actions.
- [ ] Establish permission-aware action components that explain unavailable
      actions without leaking admin-only details.
- [ ] Make mobile layouts first-class for dashboards, course pages, forms,
      tables, and navigation.
- [ ] Meet basic accessibility expectations: semantic landmarks, focus order,
      keyboard access, visible focus, labels, error associations, and color
      contrast.
- [ ] Replace test-console language with product language that matches each
      persona and task.

## Frontend Data Layer

- [ ] Create typed API helpers for authentication, users, courses,
      organizations, teacher applications, rewards, reports, fraud blocks,
      delegations, wallets, notifications, and health.
- [ ] Centralize request timeout, JSON parsing, file download handling,
      authentication headers, and error normalization.
- [ ] Add shared models for API responses used by multiple routes.
- [ ] Support pagination, filtering, sorting, and refresh behavior consistently.
- [ ] Avoid duplicating endpoint strings and permission checks across pages.
- [ ] Document frontend/backend contract gaps as issues before inventing fake
      UI behavior.

