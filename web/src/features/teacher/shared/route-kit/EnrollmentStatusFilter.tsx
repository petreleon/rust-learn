"use client";

import { enrollmentStatusOptions } from "./enrollmentStatusOptions";

export type EnrollmentStatusFilter = (typeof enrollmentStatusOptions)[number];
