"use client";

import { RefreshCw, Search } from "lucide-react";
import styles from "@/features/organization/shared/organization-routes.module.css";
import { organizationMemberPermissionOptions } from "@/features/organization/shared/route-kit/organizationMemberPermissionOptions";
import { organizationMemberRoleOptions } from "@/features/organization/shared/route-kit/organizationMemberRoleOptions";

export function MemberFilterPanel({
  draftSearch,
  onApplyFilters,
  onDraftSearchChange,
  onPermissionFilterChange,
  onResetFilters,
  onRoleFilterChange,
  permissionFilter,
  roleFilter,
}: {
  draftSearch: string;
  onApplyFilters: () => void;
  onDraftSearchChange: (value: string) => void;
  onPermissionFilterChange: (value: string) => void;
  onResetFilters: () => void;
  onRoleFilterChange: (value: string) => void;
  permissionFilter: string;
  roleFilter: string;
}) {
  return (
    <section className={styles.memberFilterPanel} aria-label="Organization member filters">
      <label>
        <span>Search members</span>
        <span className={styles.inputWithIcon}>
          <Search size={17} aria-hidden />
          <input
            onChange={(event) => onDraftSearchChange(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                onApplyFilters();
              }
            }}
            placeholder="Name, email, role, or permission"
            type="search"
            value={draftSearch}
          />
        </span>
      </label>
      <label>
        <span>Role</span>
        <select aria-label="Organization member role filter" onChange={(event) => onRoleFilterChange(event.target.value)} value={roleFilter}>
          {organizationMemberRoleOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <label>
        <span>Permission</span>
        <select
          aria-label="Organization member permission filter"
          onChange={(event) => onPermissionFilterChange(event.target.value)}
          value={permissionFilter}
        >
          {organizationMemberPermissionOptions.map((option) => (
            <option key={option.label} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <div className={styles.filterActions}>
        <button className={styles.primaryButton} onClick={onApplyFilters} type="button">
          <Search size={17} aria-hidden />
          Apply
        </button>
        <button className={styles.secondaryButton} onClick={onResetFilters} type="button">
          <RefreshCw size={17} aria-hidden />
          Reset
        </button>
      </div>
    </section>
  );
}
