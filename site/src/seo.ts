import type { SeoMetadata, SiteMetadata } from './application/site-contract';

function toAbsoluteUrl(path: string, site: SiteMetadata) {
  return new URL(path, site.baseUrl).toString();
}

function toHrefLang(locale: string) {
  return locale === 'fr-fr' ? 'fr-FR' : locale === 'en-us' ? 'en-US' : locale;
}

function upsertMeta(selector: string, attributes: Record<string, string>, content?: string) {
  let element = document.head.querySelector(selector);

  if (!element) {
    element = document.createElement('meta');
    document.head.appendChild(element);
  }

  Object.entries(attributes).forEach(([key, value]) => {
    element?.setAttribute(key, value);
  });

  if (content !== undefined) {
    element.setAttribute('content', content);
  }
}

function upsertLink(selector: string, attributes: Record<string, string>) {
  let element = document.head.querySelector(selector);

  if (!element) {
    element = document.createElement('link');
    document.head.appendChild(element);
  }

  Object.entries(attributes).forEach(([key, value]) => {
    element?.setAttribute(key, value);
  });
}

function removeManagedLinks() {
  document.head.querySelectorAll('link[data-managed="true"]').forEach((element) => element.remove());
}

export function applySeo(seo: SeoMetadata, site: SiteMetadata) {
  const description = seo.description ?? '';
  const ogTitle = seo.ogTitle ?? seo.title;
  const ogDescription = seo.ogDescription ?? description;
  const imageUrl = seo.ogImage ? toAbsoluteUrl(seo.ogImage, site) : undefined;

  document.title = seo.title;
  document.documentElement.lang = seo.lang;

  upsertMeta('meta[name="description"]', { name: 'description' }, description);
  upsertMeta('meta[name="robots"]', { name: 'robots' }, seo.robots);

  upsertMeta('meta[property="og:title"]', { property: 'og:title' }, ogTitle);
  upsertMeta('meta[property="og:description"]', { property: 'og:description' }, ogDescription);
  upsertMeta('meta[property="og:type"]', { property: 'og:type' }, 'website');

  upsertMeta('meta[name="twitter:card"]', { name: 'twitter:card' }, 'summary_large_image');
  upsertMeta('meta[name="twitter:title"]', { name: 'twitter:title' }, ogTitle);
  upsertMeta('meta[name="twitter:description"]', { name: 'twitter:description' }, ogDescription);

  if (imageUrl) {
    upsertMeta('meta[property="og:image"]', { property: 'og:image' }, imageUrl);
    upsertMeta('meta[name="twitter:image"]', { name: 'twitter:image' }, imageUrl);
  }

  removeManagedLinks();

  if (seo.canonicalPath) {
    const canonicalUrl = toAbsoluteUrl(seo.canonicalPath, site);

    upsertLink('link[rel="canonical"]', {
      rel: 'canonical',
      href: canonicalUrl,
      'data-managed': 'true',
    });

    upsertMeta('meta[property="og:url"]', { property: 'og:url' }, canonicalUrl);
  }

  seo.alternates.forEach((alternate) => {
    const link = document.createElement('link');
    link.setAttribute('rel', 'alternate');
    link.setAttribute('hreflang', toHrefLang(alternate.locale));
    link.setAttribute('href', toAbsoluteUrl(alternate.path, site));
    link.setAttribute('data-managed', 'true');
    document.head.appendChild(link);
  });

  const xDefault = document.createElement('link');
  xDefault.setAttribute('rel', 'alternate');
  xDefault.setAttribute('hreflang', 'x-default');
  xDefault.setAttribute('href', toAbsoluteUrl('/', site));
  xDefault.setAttribute('data-managed', 'true');
  document.head.appendChild(xDefault);

  const rss = document.createElement('link');
  rss.setAttribute('rel', 'alternate');
  rss.setAttribute('type', 'application/rss+xml');
  rss.setAttribute('title', `${site.name} RSS`);
  rss.setAttribute('href', toAbsoluteUrl('/rss.xml', site));
  rss.setAttribute('data-managed', 'true');
  document.head.appendChild(rss);
}
