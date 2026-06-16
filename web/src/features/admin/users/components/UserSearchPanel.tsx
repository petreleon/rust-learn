"use client";

import { Search } from "lucide-react";
import { type FormEvent } from "react";
import styles from "@/features/admin/shared/admin-routes.module.css";
import { type RouteError } from "@/shared/route-state/RouteError";

export function UserSearchPanel({
  error,
  onSearch,
  onSearchInputChange,
  searchInput,
  state,
}: {
  error: RouteError | null;
  onSearch: (event: FormEvent<HTMLFormElement>) => void;
  onSearchInputChange: (value: string) => void;
  searchInput: string;
  state: string;
}) {
  const canSearch = searchInput.trim().length > 0 && state !== "loading";

  return (
    <form className={styles.filterPanel} onSubmit={onSearch}>
      <label>
        Search users
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            aria-label="Search users"
            onChange={(event) => onSearchInputChange(event.target.value)}
            placeholder="Name or email"
            value={searchInput}
          />
        </span>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} disabled={!canSearch} type="submit">
          {state === "loading" ? "Searching" : "Search"}
        </button>
      </div>
      {error ? <p className={styles.inlineError}>{error.message}</p> : null}
    </form>
  );
}
