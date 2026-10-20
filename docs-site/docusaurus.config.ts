import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import airsTheme from './src/css/prism-airs';

const config: Config = {
  title: 'SCM SDK for Rust',
  tagline: 'Native Rust SDK for Strata Cloud Manager, beginning with Prisma AIRS',
  favicon: 'img/brand-logo.png',

  future: {
    v4: true,
  },

  url: 'https://cdot65.github.io',
  baseUrl: '/scm-sdk-rs/',

  organizationName: 'cdot65',
  projectName: 'scm-sdk-rs',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  markdown: {
    // .md → CommonMark (safe for code-heavy guides + TypeDoc output), .mdx → MDX
    format: 'detect',
    mermaid: true,
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },

  themes: ['@docusaurus/theme-mermaid'],

  // Rust product guides and reference navigation live in docs/reference/api.

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          routeBasePath: '/',
          editUrl: 'https://github.com/cdot65/scm-sdk-rs/edit/docs/scm-github-pages/docs-site/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    mermaid: {theme: {light: 'dark', dark: 'dark'}, options: {themeVariables: {
      background: '#030609', primaryColor: '#061b29', primaryTextColor: '#f5f8fa',
      primaryBorderColor: '#00ddf2', lineColor: '#8999a6', secondaryColor: '#0b293b', tertiaryColor: '#061b29',
    }}},
    image: 'img/brand-logo.png',
    // Prisma AIRS brand palette uses a dark canvas.
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: 'SCM SDK for Rust',
      logo: {
        alt: 'SCM SDK for Rust',
        src: 'img/brand-logo.png',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docs',
          position: 'left',
          label: 'Docs',
        },
        {
          to: '/reference/api/',
          label: 'API Reference',
          position: 'left',
        },
        {
          href: 'https://crates.io/crates/scm-sdk',
          label: 'crates.io',
          position: 'right',
        },
        {
          href: 'https://github.com/cdot65/scm-sdk-rs',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {label: 'Getting Started', to: '/getting-started/installation'},
            {label: 'Guides', to: '/airs/scanning'},
            {label: 'API Reference', to: '/reference/api/'},
          ],
        },
        {
          title: 'More',
          items: [
            {label: 'crates.io', href: 'https://crates.io/crates/scm-sdk'},
            {label: 'GitHub', href: 'https://github.com/cdot65/scm-sdk-rs'},
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} cdot65. Built with Docusaurus.`,
    },
    prism: {
      theme: airsTheme,
      darkTheme: airsTheme,
      additionalLanguages: ['rust', 'toml', 'bash', 'json', 'diff'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
