import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { WalletContent } from "@/components/learner-routes/WalletContent";

describe("WalletContent KYC gate", () => {
  it("disables wallet linking until KYC is verified", () => {
    render(<WalletContent kycVerified={false} linking={false} onLinkWallet={vi.fn()} onRefresh={vi.fn()} rewards={[]} wallet={null} />);

    expect(screen.getByText("Wallet action readiness")).toBeVisible();
    expect(screen.getByText("KYC required")).toBeVisible();
    expect(screen.getByRole("button", { name: "Link wallet" })).toBeDisabled();
    expect(screen.getByRole("link", { name: "Open verification" })).toHaveAttribute("href", "/settings/account");
  });

  it("enables wallet linking when KYC is verified", () => {
    render(<WalletContent kycVerified linking={false} onLinkWallet={vi.fn()} onRefresh={vi.fn()} rewards={[]} wallet={null} />);

    expect(screen.getByText("KYC verified")).toBeVisible();
    expect(screen.getByRole("button", { name: "Link wallet" })).toBeEnabled();
    expect(screen.queryByRole("link", { name: "Open verification" })).not.toBeInTheDocument();
  });
});
