import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '..');
const distDir = path.join(rootDir, 'dist');
const serverEntryPath = path.join(distDir, 'server', 'entry-server.js');
const templatePath = path.join(distDir, 'index.html');
const template = readFileSync(templatePath, 'utf-8');

const serverEntry = await import(pathToFileURL(serverEntryPath).href);
const { render, getPrerenderRoutes, getSiteContent } = serverEntry;
const source = getSiteContent();

const siteUrl = source.site.baseUrl;
const siteName = source.site.name;
const defaultLocale = source.site.defaultLocale ?? 'fr-fr';
const defaultOgImage = source.site.defaultOgImage ?? '/og/bezot-corp-default.png';

function escapeHtml(value = '') {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;');
}

function escapeXml(value = '') {
  return escapeHtml(value).replaceAll("'", '&apos;');
}

function escapeJsonForHtml(value) {
  return JSON.stringify(value)
    .replaceAll('<', '\\u003c')
    .replaceAll('>', '\\u003e')
    .replaceAll('&', '\\u0026');
}

function normalizePath(routePath) {
  if (!routePath || routePath === '/') return '/';
  return routePath.endsWith('/') ? routePath : `${routePath}/`;
}

function absoluteUrl(routePath) {
  return new URL(normalizePath(routePath), siteUrl).toString();
}

function absoluteAssetUrl(assetPath) {
  return new URL(assetPath || defaultOgImage, siteUrl).toString();
}

function toHrefLang(locale) {
  return locale === 'fr-fr' ? 'fr-FR' : locale === 'en-us' ? 'en-US' : locale;
}

function getEntries() {
  return [...(source.pages ?? []), ...(source.posts ?? [])];
}

function getPathForEntry(entry, locale) {
  const content = entry.locales[locale];
  return content.slug ? `/${locale}/${content.slug}` : `/${locale}`;
}

function findEntryForRoute(routePath) {
  const normalizedRoute = normalizePath(routePath);

  for (const entry of getEntries()) {
    for (const locale of source.site.locales) {
      const content = entry.locales[locale];
      const entryPath = getPathForEntry(entry, locale);

      if (normalizePath(entryPath) === normalizedRoute) {
        return { entry, locale, content };
      }
    }
  }

  return null;
}

function isPost(entry) {
  return (source.posts ?? []).some((post) => post.id === entry?.id);
}

function findBlogPage(locale) {
  return (source.pages ?? []).find((page) => page.id === 'blog' && page.locales?.[locale]);
}

function buildOrganizationJsonLd() {
  return {
    '@context': 'https://schema.org',
    '@type': 'Organization',
    '@id': `${siteUrl}/#organization`,
    name: siteName,
    url: siteUrl,
    logo: absoluteAssetUrl(defaultOgImage),
  };
}

function buildWebsiteJsonLd() {
  return {
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    '@id': `${siteUrl}/#website`,
    name: siteName,
    url: siteUrl,
    publisher: {
      '@id': `${siteUrl}/#organization`,
    },
    inLanguage: source.site.locales.map(toHrefLang),
  };
}

function buildArticleJsonLd(entry, locale, content, routePath, seo) {
  const image = seo.ogImage || content.seo?.ogImage || defaultOgImage;

  return {
    '@context': 'https://schema.org',
    '@type': 'Article',
    '@id': `${absoluteUrl(routePath)}#article`,
    headline: seo.ogTitle ?? seo.title,
    description: seo.ogDescription ?? seo.description,
    image: absoluteAssetUrl(image),
    datePublished: entry.publishedAt,
    dateModified: entry.updatedAt ?? entry.publishedAt,
    author: {
      '@type': 'Organization',
      name: entry.author ?? siteName,
      url: siteUrl,
    },
    publisher: {
      '@id': `${siteUrl}/#organization`,
    },
    mainEntityOfPage: {
      '@type': 'WebPage',
      '@id': absoluteUrl(routePath),
    },
    inLanguage: toHrefLang(locale),
  };
}

function buildBreadcrumbJsonLd(entry, locale, content, routePath, seo) {
  if (routePath === '/') {
    return null;
  }

  const items = [
    {
      '@type': 'ListItem',
      position: 1,
      name: locale === 'fr-fr' ? 'Accueil' : 'Home',
      item: absoluteUrl(`/${locale}/`),
    },
  ];

  const blogPage = findBlogPage(locale);
  const slug = content.slug ?? '';

  if (blogPage && slug.startsWith('blog/') && slug !== 'blog') {
    items.push({
      '@type': 'ListItem',
      position: 2,
      name: 'Blog',
      item: absoluteUrl(getPathForEntry(blogPage, locale)),
    });
  }

  items.push({
    '@type': 'ListItem',
    position: items.length + 1,
    name: seo.ogTitle ?? seo.title,
    item: absoluteUrl(routePath),
  });

  return {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: items,
  };
}

function buildJsonLdTags(routePath, seo) {
  const matched = findEntryForRoute(routePath);
  const jsonLd = [buildOrganizationJsonLd(), buildWebsiteJsonLd()];

  if (matched) {
    const breadcrumb = buildBreadcrumbJsonLd(matched.entry, matched.locale, matched.content, routePath, seo);

    if (breadcrumb) {
      jsonLd.push(breadcrumb);
    }

    if (isPost(matched.entry)) {
      jsonLd.push(buildArticleJsonLd(matched.entry, matched.locale, matched.content, routePath, seo));
    }
  }

  return jsonLd
    .map(
      (item) =>
        `    <script type="application/ld+json">${escapeJsonForHtml(item)}</script>`,
    )
    .join('\n');
}

function buildHeadTags(seo, routePath) {
  const tags = [
    `<meta name="description" content="${escapeHtml(seo.description ?? '')}">`,
    `<meta name="robots" content="${escapeHtml(seo.robots ?? 'index, follow')}">`,
    `<link rel="alternate" type="application/rss+xml" title="${escapeHtml(siteName)} RSS" href="${escapeHtml(
      new URL('/rss.xml', siteUrl).toString(),
    )}">`,
  ];

  if (seo.canonicalPath) {
    const canonicalUrl = absoluteUrl(seo.canonicalPath);
    tags.push(`<link rel="canonical" href="${escapeHtml(canonicalUrl)}">`);
    tags.push(`<meta property="og:url" content="${escapeHtml(canonicalUrl)}">`);
  }

  seo.alternates.forEach((alternate) => {
    tags.push(
      `<link rel="alternate" hreflang="${toHrefLang(alternate.locale)}" href="${escapeHtml(
        absoluteUrl(alternate.path),
      )}">`,
    );
  });

  tags.push(`<link rel="alternate" hreflang="x-default" href="${escapeHtml(absoluteUrl('/'))}">`);

  tags.push(`<meta property="og:title" content="${escapeHtml(seo.ogTitle ?? seo.title)}">`);

  if (seo.ogDescription ?? seo.description) {
    tags.push(`<meta property="og:description" content="${escapeHtml(seo.ogDescription ?? seo.description)}">`);
  }

  tags.push(`<meta property="og:type" content="${findEntryForRoute(routePath)?.entry && isPost(findEntryForRoute(routePath).entry) ? 'article' : 'website'}">`);

  if (seo.ogImage) {
    tags.push(`<meta property="og:image" content="${escapeHtml(absoluteAssetUrl(seo.ogImage))}">`);
  }

  tags.push(`<meta name="twitter:card" content="summary_large_image">`);
  tags.push(`<meta name="twitter:title" content="${escapeHtml(seo.ogTitle ?? seo.title)}">`);

  if (seo.ogDescription ?? seo.description) {
    tags.push(`<meta name="twitter:description" content="${escapeHtml(seo.ogDescription ?? seo.description)}">`);
  }

  if (seo.ogImage) {
    tags.push(`<meta name="twitter:image" content="${escapeHtml(absoluteAssetUrl(seo.ogImage))}">`);
  }

  const htmlTags = tags.map((tag) => `    ${tag}`).join('\n');
  const jsonLdTags = buildJsonLdTags(routePath, seo);

  return `${htmlTags}\n${jsonLdTags}`;
}

function renderDocument(routePath) {
  const { appHtml, seo } = render(routePath);

  return template
    .replace(/<html lang="[^"]*">/, `<html lang="${escapeHtml(seo.lang)}">`)
    .replace(/<title>.*?<\/title>/, `<title>${escapeHtml(seo.title)}</title>`)
    .replace('<div id="root"></div>', `<div id="root">${appHtml}</div>`)
    .replace('</head>', `${buildHeadTags(seo, routePath)}\n  </head>`);
}

function writeRoute(routePath, html) {
  const trimmedPath = normalizePath(routePath).replace(/^\//, '').replace(/\/$/, '');
  const routeDir = trimmedPath ? path.join(distDir, trimmedPath) : distDir;

  mkdirSync(routeDir, { recursive: true });
  writeFileSync(path.join(routeDir, 'index.html'), html);
}

function findUpdatedAt(routePath) {
  const matched = findEntryForRoute(routePath);
  return matched?.content.updatedAt ?? matched?.entry.updatedAt;
}

function buildSitemapAlternateLinks(route) {
  const links = route.seo.alternates.map(
    (alternate) =>
      `    <xhtml:link rel="alternate" hreflang="${toHrefLang(alternate.locale)}" href="${escapeXml(
        absoluteUrl(alternate.path),
      )}" />`,
  );

  links.push(`    <xhtml:link rel="alternate" hreflang="x-default" href="${escapeXml(absoluteUrl('/'))}" />`);

  return links.join('\n');
}

function buildSitemap(routes) {
  const entries = routes
    .map((route) => {
      const alternateLinks = buildSitemapAlternateLinks(route);
      const lastmod = findUpdatedAt(route.path);
      const lastmodLine = lastmod ? `\n    <lastmod>${escapeXml(lastmod)}</lastmod>` : '';

      return `  <url>
    <loc>${escapeXml(absoluteUrl(route.path))}</loc>${lastmodLine}
${alternateLinks}
  </url>`;
    })
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${entries}
</urlset>
`;
}

function getRssPosts(locale = defaultLocale) {
  return (source.posts ?? [])
    .filter((post) => post.status === 'published' && post.locales?.[locale])
    .slice()
    .sort((a, b) => String(b.publishedAt ?? '').localeCompare(String(a.publishedAt ?? '')));
}

function getPlainTextFromBlocks(blocks = []) {
  return blocks
    .filter((block) => block.type === 'paragraph' && typeof block.props?.text === 'string')
    .map((block) => block.props.text)
    .join('\n\n');
}

function buildRssFeed(locale = defaultLocale) {
  const posts = getRssPosts(locale);
  const blogPage = findBlogPage(locale);
  const blogUrl = blogPage ? absoluteUrl(getPathForEntry(blogPage, locale)) : absoluteUrl(`/${locale}/blog/`);

  const items = posts
    .map((post) => {
      const content = post.locales[locale];
      const postUrl = absoluteUrl(getPathForEntry(post, locale));
      const description = content.seo?.description ?? content.seo?.ogDescription ?? '';
      const body = getPlainTextFromBlocks(content.blocks ?? []);
      const pubDate = post.publishedAt ? new Date(post.publishedAt).toUTCString() : new Date().toUTCString();

      return `    <item>
      <title>${escapeXml(content.seo?.ogTitle ?? content.seo?.title ?? post.id)}</title>
      <link>${escapeXml(postUrl)}</link>
      <guid isPermaLink="true">${escapeXml(postUrl)}</guid>
      <pubDate>${escapeXml(pubDate)}</pubDate>
      <description>${escapeXml(description)}</description>
      <content:encoded><![CDATA[${body}]]></content:encoded>
    </item>`;
    })
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>${escapeXml(siteName)}</title>
    <link>${escapeXml(blogUrl)}</link>
    <description>${escapeXml(locale === 'fr-fr' ? 'Articles et notes de Bezot Corp.' : 'Articles and notes from Bezot Corp.')}</description>
    <language>${escapeXml(toHrefLang(locale))}</language>
    <lastBuildDate>${escapeXml(new Date().toUTCString())}</lastBuildDate>
${items}
  </channel>
</rss>
`;
}

function escapeRewritePath(routePath) {
  return normalizePath(routePath)
    .replace(/^\//, '')
    .replace(/\/$/, '')
    .replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function buildHtaccess(routes) {
  const lines = [
    'RewriteEngine On',
    '',
    '# Existing files and directories',
    'RewriteCond %{REQUEST_FILENAME} -f [OR]',
    'RewriteCond %{REQUEST_FILENAME} -d',
    'RewriteRule ^ - [L]',
    '',
    '# Explicit redirects',
  ];

  for (const redirect of source.redirects ?? []) {
    if (redirect.from === '/') continue;

    const from = escapeRewritePath(redirect.from);
    const to = normalizePath(redirect.to);
    const status = redirect.status ?? 301;

    lines.push(`RewriteRule ^${from}/?$ ${to} [R=${status},L]`);
  }

  lines.push('', '# Gone URLs');

  for (const gonePath of source.gone ?? []) {
    const from = escapeRewritePath(gonePath);
    lines.push(`RewriteRule ^${from}/?$ - [G,L]`);
  }

  lines.push('', '# Known prerendered routes');

  for (const route of routes) {
    const routePattern = escapeRewritePath(route.path);

    if (!routePattern) {
      lines.push('RewriteRule ^$ /index.html [L]');
      continue;
    }

    lines.push(`RewriteRule ^${routePattern}/?$ /${routePattern}/index.html [L]`);
  }

  lines.push('', '# Real 404 for unknown URLs', 'ErrorDocument 404 /404.html', 'RewriteRule ^ - [R=404,L]', '');

  return `${lines.join('\n')}\n`;
}

const routes = getPrerenderRoutes();

for (const route of routes) {
  writeRoute(route.path, renderDocument(route.path));
}

writeFileSync(path.join(distDir, '404.html'), renderDocument('/404'));
writeFileSync(path.join(distDir, 'sitemap.xml'), buildSitemap(routes));
writeFileSync(path.join(distDir, 'rss.xml'), buildRssFeed(defaultLocale));
writeFileSync(path.join(distDir, 'feed.xml'), buildRssFeed(defaultLocale));
writeFileSync(
  path.join(distDir, 'robots.txt'),
  `User-agent: *\nAllow: /\n\nSitemap: ${new URL('/sitemap.xml', siteUrl).toString()}\n`,
);
writeFileSync(path.join(distDir, '.htaccess'), buildHtaccess(routes));

rmSync(path.join(distDir, 'server'), { recursive: true, force: true });

console.log('Prerender complete');
