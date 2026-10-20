import React, {type ReactNode} from 'react';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './index.module.css';
const paths = [["01", "Make your first request", "Install the SDK, configure credentials, and run a content scan.", "/getting-started/quick-start"], ["02", "Connect AI Gateway", "Keep runtime inference and management credentials clearly separated.", "/airs/gateway-runtime"], ["03", "Explore the APIs", "Find typed clients, request models, methods, and runnable examples.", "/reference/api/"], ["04", "Build a security workflow", "Use practical examples across scanning, red teaming, and model security.", "/developer/examples"]];
export default function Home(): ReactNode {
  return <Layout title="SCM SDK for Rust" description="Native Rust clients for Strata Cloud Manager. Start with Prisma AIRS: scan content, manage security, test models, and connect AI Gateway. NGFW and SASE support are planned."><main>
    <section className={styles.hero} aria-labelledby="hero-title">
      <div><p className={styles.eyebrow}>STRATA CLOUD MANAGER / RUST SDK</p><h1 id="hero-title">Build with confidence.<br /><span>Secure every interaction.</span></h1>
      <p className={styles.lead}>Native Rust clients for Strata Cloud Manager. Start with Prisma AIRS: scan content, manage security, test models, and connect AI Gateway. NGFW and SASE support are planned.</p><div className={styles.actions}><Link className="button button--primary button--lg" to="/getting-started/installation">Get started →</Link><Link className={styles.secondary} to="/reference/api/">API reference ↗</Link></div><p className={styles.platforms}>RUST · ASYNC · APPLICATION-OWNED RUNTIME</p></div>
      <div className={styles.artwork}><img src={useBaseUrl('/img/brand-logo.png')} alt="SCM SDK for Rust shield and prism spectrum" width="1254" height="1254" fetchPriority="high" /><div className={styles.pillRow}><span className={styles.pill}>Secure by design</span><span className={styles.pill}>Developer first</span></div></div>
    </section>
    <section className={styles.paths} aria-labelledby="paths-title"><div className={styles.sectionIntro}><p className={styles.eyebrow}>BUILD · CONNECT · PROTECT · SCALE</p><h2 id="paths-title">Turn intent into action.</h2><p>Choose a starting point. Go from your first request to a repeatable security workflow.</p></div><div className={styles.grid}>{paths.map(([number,title,description,to]) => <Link className={styles.path} to={to} key={number}><span className={styles.number}>{number}</span><h3>{title}</h3><p>{description}</p><span className={styles.arrow} aria-hidden="true">↗</span></Link>)}</div></section>
    <section className={styles.quick}><div><p className={styles.eyebrow}>ONE SCM SDK. ROOM TO GROW.</p><h2>Build a safer tomorrow.</h2><p>Prisma AIRS is available today. Explore shared SCM services and the planned NGFW and SASE product areas.</p></div><div className={styles.actions}><Link to="/products/overview">Prisma AIRS →</Link><Link to="/products/ngfw">NGFW · planned →</Link><Link to="/products/sase">SASE · planned →</Link></div></section>
  </main></Layout>;
}
