import styles from "./page.module.css";

async function getHello() {
  const apiUrl = process.env.API_URL || "http://app:8080";
  try {
    const res = await fetch(apiUrl, { cache: "no-store" });
    if (!res.ok) {
      throw new Error("Failed to fetch data from API");
    }
    return res.text();
  } catch (error) {
    console.error("API Fetch Error:", error);
    return "Could not connect to API";
  }
}

export default async function Home() {
  const message = await getHello();
  const isError = message === "Could not connect to API";

  return (
    <div className={styles.container}>
      <header className={styles.hero}>
        <h1 className={styles.gradientText}>Rust-Learn Platform</h1>
        <p>Your journey into learning Rust starts here. A modern backend paired with a dynamic frontend experience.</p>
      </header>

      <main className={styles.main}>
        <div className={styles.grid}>
          <section className={styles.card}>
            <h2>
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>
              API Status
            </h2>
            <div className={styles.apiStatusContent}>
              <div className={`${styles.statusIndicator} ${isError ? styles.error : ''}`}></div>
              <div className={styles.messageBox}>
                <p>Latest Message</p>
                <strong>{message}</strong>
              </div>
            </div>
          </section>

          <section className={styles.card}>
            <h2>
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/></svg>
              Resources
            </h2>
            <ul className={styles.linksList}>
              <li className={styles.linkItem}>
                <a href="https://nextjs.org/docs" target="_blank" rel="noopener noreferrer">
                  Next.js Documentation
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
                </a>
              </li>
              <li className={styles.linkItem}>
                <a href="https://rust-lang.org" target="_blank" rel="noopener noreferrer">
                  Rust Language
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline></svg>
                </a>
              </li>
            </ul>
          </section>
        </div>
      </main>

      <footer className={styles.footer}>
        <p>&copy; {new Date().getFullYear()} Rust-Learn Platform. All rights reserved.</p>
      </footer>
    </div>
  );
}
