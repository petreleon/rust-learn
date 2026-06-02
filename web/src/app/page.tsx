"use client";

import { useEffect, useMemo, useState } from "react";
import styles from "./page.module.css";

type ApiState = "checking" | "online" | "offline";

type CourseProgress = {
  title: string;
  organization: string;
  nextStep: string;
  progress: number;
  reward: string;
  status: "active" | "review" | "completed";
};

type LearnerTask = {
  label: string;
  course: string;
  due: string;
  status: "ready" | "locked" | "submitted";
};

type AdminWorkItem = {
  label: string;
  owner: string;
  state: "attention" | "queued" | "healthy";
  metric: string;
};

const learnerCourses: CourseProgress[] = [
  {
    title: "Rust ownership foundations",
    organization: "Core Systems Guild",
    nextStep: "Borrowing checkpoint",
    progress: 68,
    reward: "42 LRN pending",
    status: "active",
  },
  {
    title: "Async service design",
    organization: "Backend Academy",
    nextStep: "Worker retry lab",
    progress: 36,
    reward: "18 LRN available",
    status: "review",
  },
  {
    title: "Smart-contract rewards",
    organization: "Token Lab",
    nextStep: "Permit signing quiz",
    progress: 100,
    reward: "100 LRN recorded",
    status: "completed",
  },
];

const learnerTasks: LearnerTask[] = [
  {
    label: "Submit ownership quiz",
    course: "Rust ownership foundations",
    due: "Today",
    status: "ready",
  },
  {
    label: "Watch retry-state walkthrough",
    course: "Async service design",
    due: "Jun 4",
    status: "ready",
  },
  {
    label: "Claim reward after audit",
    course: "Smart-contract rewards",
    due: "Ready",
    status: "submitted",
  },
];

const adminWork: AdminWorkItem[] = [
  {
    label: "Review pending course publication",
    owner: "Core Systems Guild",
    state: "attention",
    metric: "3 chapters",
  },
  {
    label: "Approve role assignment",
    owner: "Backend Academy",
    state: "queued",
    metric: "2 users",
  },
  {
    label: "Reconcile reward events",
    owner: "Token Lab",
    state: "healthy",
    metric: "12 events",
  },
];

const notifications = [
  "New content was published in Async service design.",
  "You were assigned STUDENT in Token Lab.",
  "Reward transfer recorded for Smart-contract rewards.",
];

function statusLabel(status: ApiState) {
  if (status === "online") {
    return "API online";
  }
  if (status === "offline") {
    return "API offline";
  }
  return "Checking API";
}

function useApiStatus() {
  const [state, setState] = useState<ApiState>("checking");
  const [message, setMessage] = useState("Connecting to RustLearn API");

  useEffect(() => {
    const controller = new AbortController();
    const apiRoot = process.env.NEXT_PUBLIC_API_URL || "/api";
    const target = apiRoot.endsWith("/") ? apiRoot : `${apiRoot}/`;

    fetch(target, { signal: controller.signal })
      .then(async (response) => {
        if (!response.ok) {
          throw new Error(`HTTP ${response.status}`);
        }
        const body = await response.text();
        setState("online");
        setMessage(body || "API responded");
      })
      .catch((error: Error) => {
        if (controller.signal.aborted) {
          return;
        }
        setState("offline");
        setMessage(error.message || "Connection failed");
      });

    return () => controller.abort();
  }, []);

  return { state, message };
}

export default function Home() {
  const api = useApiStatus();
  const activeCourses = useMemo(
    () => learnerCourses.filter((course) => course.status !== "completed").length,
    []
  );
  const pendingRewards = learnerCourses
    .filter((course) => course.reward.includes("pending"))
    .map((course) => course.reward)
    .join(", ");

  return (
    <main className={styles.shell}>
      <aside className={styles.sidebar} aria-label="Workspace navigation">
        <div className={styles.brand}>
          <span className={styles.brandMark}>RL</span>
          <div>
            <p className={styles.brandName}>RustLearn</p>
            <p className={styles.brandMeta}>Learning operations</p>
          </div>
        </div>
        <nav className={styles.navList}>
          <a className={styles.navItemActive} href="#learner">
            Learner
          </a>
          <a className={styles.navItem} href="#administrator">
            Administrator
          </a>
          <a className={styles.navItem} href="#activity">
            Activity
          </a>
        </nav>
        <div className={styles.sidebarStatus}>
          <span className={`${styles.statusDot} ${styles[api.state]}`} />
          <div>
            <p>{statusLabel(api.state)}</p>
            <span>{api.message}</span>
          </div>
        </div>
      </aside>

      <section className={styles.workspace}>
        <header className={styles.topbar}>
          <div>
            <p className={styles.eyebrow}>Workspace</p>
            <h1>Learning and administration</h1>
          </div>
          <div className={styles.topbarActions}>
            <button type="button" className={styles.secondaryButton}>
              Export
            </button>
            <button type="button" className={styles.primaryButton}>
              Create course
            </button>
          </div>
        </header>

        <section className={styles.metrics} aria-label="Workspace metrics">
          <div className={styles.metric}>
            <span>Active courses</span>
            <strong>{activeCourses}</strong>
          </div>
          <div className={styles.metric}>
            <span>Pending rewards</span>
            <strong>{pendingRewards || "None"}</strong>
          </div>
          <div className={styles.metric}>
            <span>Admin queue</span>
            <strong>{adminWork.length} items</strong>
          </div>
        </section>

        <div className={styles.columns}>
          <section id="learner" className={styles.panel} aria-labelledby="learner-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Learner</p>
                <h2 id="learner-title">My learning path</h2>
              </div>
              <button type="button" className={styles.ghostButton}>
                View all
              </button>
            </div>

            <div className={styles.courseList}>
              {learnerCourses.map((course) => (
                <article key={course.title} className={styles.courseRow}>
                  <div className={styles.courseMain}>
                    <div>
                      <h3>{course.title}</h3>
                      <p>{course.organization}</p>
                    </div>
                    <span className={`${styles.pill} ${styles[course.status]}`}>
                      {course.status}
                    </span>
                  </div>
                  <div className={styles.progressTrack} aria-label={`${course.progress}% complete`}>
                    <span style={{ width: `${course.progress}%` }} />
                  </div>
                  <div className={styles.courseFooter}>
                    <span>{course.nextStep}</span>
                    <strong>{course.reward}</strong>
                  </div>
                </article>
              ))}
            </div>
          </section>

          <section className={styles.panel} aria-labelledby="tasks-title">
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Learner</p>
                <h2 id="tasks-title">Task queue</h2>
              </div>
              <button type="button" className={styles.ghostButton}>
                Start
              </button>
            </div>
            <ul className={styles.taskList}>
              {learnerTasks.map((task) => (
                <li key={task.label} className={styles.taskItem}>
                  <span className={`${styles.statusDot} ${styles[task.status]}`} />
                  <div>
                    <strong>{task.label}</strong>
                    <p>{task.course}</p>
                  </div>
                  <time>{task.due}</time>
                </li>
              ))}
            </ul>
          </section>
        </div>

        <section
          id="administrator"
          className={styles.panel}
          aria-labelledby="administrator-title"
        >
          <div className={styles.panelHeader}>
            <div>
              <p className={styles.eyebrow}>Administrator</p>
              <h2 id="administrator-title">Operations board</h2>
            </div>
            <div className={styles.segmented}>
              <button type="button" className={styles.segmentActive}>
                Today
              </button>
              <button type="button" className={styles.segment}>
                Week
              </button>
            </div>
          </div>

          <div className={styles.adminGrid}>
            {adminWork.map((item) => (
              <article key={item.label} className={styles.adminItem}>
                <span className={`${styles.stateBar} ${styles[item.state]}`} />
                <div>
                  <h3>{item.label}</h3>
                  <p>{item.owner}</p>
                </div>
                <strong>{item.metric}</strong>
                <button type="button" className={styles.secondaryButton}>
                  Open
                </button>
              </article>
            ))}
          </div>
        </section>

        <section id="activity" className={styles.activityBand} aria-labelledby="activity-title">
          <div className={styles.activityPanel}>
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Activity</p>
                <h2 id="activity-title">Notifications</h2>
              </div>
              <button type="button" className={styles.ghostButton}>
                Mark read
              </button>
            </div>
            <ul className={styles.notificationList}>
              {notifications.map((notification) => (
                <li key={notification}>{notification}</li>
              ))}
            </ul>
          </div>

          <div className={styles.activityPanel}>
            <div className={styles.panelHeader}>
              <div>
                <p className={styles.eyebrow}>Finance</p>
                <h2>Wallet review</h2>
              </div>
              <button type="button" className={styles.ghostButton}>
                Reconcile
              </button>
            </div>
            <dl className={styles.walletList}>
              <div>
                <dt>Learner wallet links</dt>
                <dd>128 active</dd>
              </div>
              <div>
                <dt>Organization wallets</dt>
                <dd>14 ready</dd>
              </div>
              <div>
                <dt>Reward audit gap</dt>
                <dd>0 events</dd>
              </div>
            </dl>
          </div>
        </section>
      </section>
    </main>
  );
}
