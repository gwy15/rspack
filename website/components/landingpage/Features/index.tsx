import styles from './index.module.scss';

const Features = () => {
  return (
    <div className={styles.featuresContainer}>
      <div className={styles.featuresContainerInner}>
        <div className={styles.features}>
          <div className={`${styles.featureCard} ${styles.whyRspack}`}>
            <div className={styles.featureContent}>
              <h3 className={styles.whyRspackText}>Why Rspack?</h3>
              <p className={styles.whyRspackDescription}>
                Rspack is a high-performance JavaScript bundler in Rust,
                compatible with the webpack ecosystem and offering
                lightning-fast build speeds.
              </p>
            </div>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>🚀</div>
            <div className={styles.featureContent}>
              <h3 className={styles.featureTitle}>Fast Startup</h3>
              <p className={styles.featureDescription}>
                Combining TypeScript and Rust with a parallelized architecture
                to bring you the ultimate developer experience.
              </p>
            </div>
          </div>
        </div>
        <div className={styles.features}>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>⚡</div>
            <div className={styles.featureContent}>
              <h3 className={styles.featureTitle}>Lightning HMR</h3>
              <p className={styles.featureDescription}>
                A built-in incremental compilation mechanism provides superior
                Hot Module Replacement performance for large-scale projects.
              </p>
            </div>
          </div>

          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>✔️</div>
            <div className={styles.featureContent}>
              <h3 className={styles.featureTitle}>Framework Agnostic</h3>
              <p className={styles.featureDescription}>
                Not bound to any frontend framework. Everyone can use it!
              </p>
            </div>
          </div>
          <div className={styles.featureCard}>
            <div className={styles.featureIcon}>🔄</div>
            <div className={styles.featureContent}>
              <h3 className={styles.featureTitle}>Webpack Compatible</h3>
              <p className={styles.featureDescription}>
                First-class support for Module Federation to facilitate the
                development of large-scale web applications.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Features;
