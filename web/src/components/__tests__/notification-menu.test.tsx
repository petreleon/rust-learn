import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { NotificationMenu } from "@/components/product-shell/NotificationMenu";
import {
  clearNotifications,
  fetchNotifications,
  markNotificationRead,
  readStoredSessionToken,
  type NotificationItem,
} from "@/lib/session";

vi.mock("@/lib/session", () => ({
  clearNotifications: vi.fn(),
  fetchNotifications: vi.fn(),
  markNotificationRead: vi.fn(),
  readStoredSessionToken: vi.fn(),
}));

function item(overrides: Partial<NotificationItem> = {}): NotificationItem {
  return {
    body: "A course update is ready.",
    created_at: "2026-06-12T11:00:00Z",
    id: 9,
    read: false,
    title: "Course update",
    user_id: 1,
    ...overrides,
  };
}

describe("NotificationMenu", () => {
  afterEach(() => {
    vi.clearAllMocks();
  });

  it("loads notifications when opened and marks unread rows as read", async () => {
    vi.mocked(readStoredSessionToken).mockReturnValue("session-token");
    vi.mocked(fetchNotifications).mockResolvedValue([item()]);
    const user = userEvent.setup();

    render(<NotificationMenu isSignedIn />);
    await user.click(screen.getByLabelText("Notifications"));

    expect(await screen.findByText("Course update")).toBeVisible();
    expect(screen.getByText("A course update is ready.")).toBeVisible();
    expect(screen.getByLabelText("1 unread notifications")).toBeVisible();

    await user.click(screen.getByRole("button", { name: "Mark read" }));

    expect(markNotificationRead).toHaveBeenCalledWith({ notificationId: 9, token: "session-token" });
    expect(screen.queryByRole("button", { name: "Mark read" })).not.toBeInTheDocument();
    expect(screen.getByLabelText("Notifications")).toBeVisible();
  });

  it("clears loaded notifications", async () => {
    vi.mocked(readStoredSessionToken).mockReturnValue("session-token");
    vi.mocked(fetchNotifications).mockResolvedValue([item({ id: 12, title: "Reward credited" })]);
    const user = userEvent.setup();

    render(<NotificationMenu isSignedIn variant="row" />);
    await user.click(screen.getByLabelText("Notifications"));
    expect(await screen.findByText("Reward credited")).toBeVisible();

    await user.click(screen.getByRole("button", { name: "Clear all" }));

    expect(clearNotifications).toHaveBeenCalledWith({ token: "session-token" });
    expect(screen.getByText("No notifications yet.")).toBeVisible();
  });
});
