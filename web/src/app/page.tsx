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

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <h1>Rust-Learn Platform</h1>
        <p>Your journey into learning Rust starts here.</p>
      </header>

      <main className={styles.main}>
        <section className={styles.apiStatus}>
          <h2>API Connection Status</h2>
          <div className={styles.card}>
            <p><strong>Message from API:</strong> {message}</p>
          </div>
        </section>

        <section className={styles.links}>
          <h2>Useful Resources</h2>
          <ul>
            <li>
              <a href="https://nextjs.org/docs" target="_blank" rel="noopener noreferrer">
                Next.js Documentation
              </a>
            </li>
            <li>
              <a href="https://rust-lang.org" target="_blank" rel="noopener noreferrer">
                Rust Language
              </a>
            </li>
          </ul>
        </section>
      </main>

      <footer className={styles.footer}>
        <p>&copy; 2026 Rust-Learn Placeholder</p>
      </footer>
    </div>
  );
}
