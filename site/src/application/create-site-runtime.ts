import type {
  BlockRegistry,
  PrerenderRoute,
  RouteMatch,
  SeoMetadata,
  SiteContent,
  SiteEntry,
  SiteEntryContent,
  SitePost,
  SiteRuntime,
} from './site-contract';

function normalizePathname(pathname: string) {
  if (!pathname || pathname === '/') {
    return '/';
  }

  return pathname.endsWith('/') ? pathname : `${pathname}/`;
}

export function createSiteRuntime(content: SiteContent, blocks: BlockRegistry): SiteRuntime {
  const { pages, posts, site: metadata } = content;
  const entries: readonly SiteEntry[] = [...pages, ...posts];
  const postEntries = new Set<SiteEntry>(posts);

  function getPathForLocaleAndSlug(locale: string, slug: string) {
    return normalizePathname(slug ? `/${locale}/${slug}` : `/${locale}`);
  }

  function isLocale(value: string | undefined) {
    return typeof value === 'string' && metadata.locales.includes(value);
  }

  function isPostEntry(entry: SiteEntry): entry is SitePost {
    return postEntries.has(entry);
  }

  function getEntryStatus(entry: SiteEntry, locale: string) {
    return isPostEntry(entry) ? entry.status : entry.locales[locale]?.status;
  }

  function isEntryPublished(entry: SiteEntry, locale: string) {
    return getEntryStatus(entry, locale) === 'published';
  }

  function findEntryByLocaleAndSlug(locale: string, slug = '') {
    const normalizedSlug = slug.replace(/^\/+|\/+$/g, '');
    const entry = entries.find(
      (candidate) =>
        isEntryPublished(candidate, locale) &&
        candidate.locales[locale]?.slug === normalizedSlug,
    );
    const entryContent = entry?.locales[locale];

    return entry && entryContent ? { entry, content: entryContent } : null;
  }

  function getLang(locale: string) {
    return locale.split('-')[0] || metadata.defaultLocale.split('-')[0] || 'en';
  }

  function getEntrySeo(entry: SiteEntry, locale: string): SeoMetadata {
    const entryContent = entry.locales[locale] as SiteEntryContent;
    const alternates = metadata.locales
      .filter((entryLocale) => isEntryPublished(entry, entryLocale))
      .map((entryLocale) => ({
        locale: entryLocale,
        path: getPathForLocaleAndSlug(entryLocale, entry.locales[entryLocale]?.slug ?? ''),
      }));

    return {
      title: entryContent.seo.title,
      description: entryContent.seo.description,
      robots: isEntryPublished(entry, locale) ? 'index, follow' : 'noindex, nofollow',
      lang: getLang(locale),
      canonicalPath: getPathForLocaleAndSlug(locale, entryContent.slug),
      alternates,
      ogTitle: entryContent.seo.ogTitle,
      ogDescription: entryContent.seo.ogDescription,
      ogImage: entryContent.seo.ogImage || metadata.defaultOgImage,
    };
  }

  function getRootSeo(): SeoMetadata {
    return {
      title: metadata.name,
      description: `Choose your language to visit ${metadata.name}.`,
      robots: 'index, follow',
      lang: getLang(metadata.defaultLocale),
      canonicalPath: '/',
      alternates: metadata.locales.map((locale) => ({
        locale,
        path: getPathForLocaleAndSlug(locale, ''),
      })),
      ogTitle: metadata.name,
      ogDescription: `Choose your language to visit ${metadata.name}.`,
      ogImage: metadata.defaultOgImage,
    };
  }

  function getNotFoundSeo(locale: string): SeoMetadata {
    const isFrench = locale === 'fr-fr';
    const title = isFrench
      ? `Page introuvable - ${metadata.name}`
      : `Page not found - ${metadata.name}`;
    const description = isFrench
      ? 'Cette URL ne correspond à aucune page publiée.'
      : 'This URL does not match any published page.';

    return {
      title,
      description,
      robots: 'noindex, nofollow',
      lang: getLang(locale),
      alternates: [],
      ogTitle: title,
      ogDescription: description,
      ogImage: metadata.defaultOgImage,
    };
  }

  function resolveRoute(pathname: string): RouteMatch {
    const normalizedPath = normalizePathname(pathname);
    const segments = normalizedPath.split('/').filter(Boolean);

    if (segments.length === 0) {
      return {
        kind: 'root',
        locale: metadata.defaultLocale,
        path: '/',
        seo: getRootSeo(),
      };
    }

    const [localeSegment, ...slugSegments] = segments;

    if (!isLocale(localeSegment)) {
      return {
        kind: 'not-found',
        locale: metadata.defaultLocale,
        path: normalizedPath,
        seo: getNotFoundSeo(metadata.defaultLocale),
      };
    }

    const entryMatch = findEntryByLocaleAndSlug(localeSegment, slugSegments.join('/'));

    if (!entryMatch) {
      return {
        kind: 'not-found',
        locale: localeSegment,
        path: normalizedPath,
        seo: getNotFoundSeo(localeSegment),
      };
    }

    return {
      kind: 'page',
      locale: localeSegment,
      path: getPathForLocaleAndSlug(localeSegment, entryMatch.content.slug),
      entry: entryMatch.entry,
      content: entryMatch.content,
      seo: getEntrySeo(entryMatch.entry, localeSegment),
    };
  }

  function getPagePath(pageId: string, locale: string) {
    const page = pages.find(
      (candidate) => candidate.id === pageId && isEntryPublished(candidate, locale),
    );
    const pageContent = page?.locales[locale];
    return pageContent ? getPathForLocaleAndSlug(locale, pageContent.slug) : null;
  }

  function getPostPath(postId: string, locale: string) {
    const post = posts.find(
      (candidate) => candidate.id === postId && isEntryPublished(candidate, locale),
    );
    const postContent = post?.locales[locale];
    return postContent ? getPathForLocaleAndSlug(locale, postContent.slug) : null;
  }

  function getPrerenderRoutes(): readonly PrerenderRoute[] {
    return [
      { path: '/', seo: getRootSeo() },
      ...entries.flatMap((entry) =>
        metadata.locales
          .filter((locale) => isEntryPublished(entry, locale))
          .map((locale) => ({
            path: getPathForLocaleAndSlug(locale, entry.locales[locale]?.slug ?? ''),
            seo: getEntrySeo(entry, locale),
          })),
      ),
    ];
  }

  return {
    content,
    blocks,
    metadata,
    defaultLocale: metadata.defaultLocale,
    isLocale,
    getActiveLocale(pathname) {
      const firstSegment = pathname.split('/').filter(Boolean)[0];
      return isLocale(firstSegment) ? firstSegment : metadata.defaultLocale;
    },
    isPostEntry,
    resolveRoute,
    getPagePath,
    getPostPath,
    getPublishedPosts() {
      return posts.filter((post) => post.status === 'published');
    },
    getPathForLocaleAndSlug,
    getNotFoundSeo,
    getPrerenderRoutes,
  };
}
