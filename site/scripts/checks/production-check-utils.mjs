import { readFileSync } from 'node:fs';
import path from 'node:path';

import {
  toPosix,
} from '../project-file-utils.mjs';
import {
  extractAttribute,
} from '../project-html-data.mjs';

export const DIST_DIR = 'dist';
export const CONTENT_DIR = 'content';
export const INVARIANTS_PATH = 'content/site-output-invariants.json';


export function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

export function readText(filePath) {
  return readFileSync(filePath, 'utf8');
}

export function startErrorCollection() {
  globalThis.__productionCheckErrors = [];
}

export function fail(message) {
  if (Array.isArray(globalThis.__productionCheckErrors)) {
    globalThis.__productionCheckErrors.push(message);
    return;
  }

  throw new Error(message);
}

export function finishErrorCollection(label) {
  const errors = globalThis.__productionCheckErrors ?? [];
  globalThis.__productionCheckErrors = undefined;

  if (errors.length > 0) {
    throw new Error(`${label} failed with ${errors.length} error(s):\n- ${errors.join('\n- ')}`);
  }
}

export function readInvariants() {
  return readJson(INVARIANTS_PATH);
}

export function htmlFileToLocation(filePath, canonicalHost) {
  const relativePath = toPosix(path.relative(DIST_DIR, filePath));

  if (relativePath === 'index.html') {
    return `${canonicalHost}/`;
  }

  if (!relativePath.endsWith('/index.html')) {
    return null;
  }

  const routePath = relativePath.slice(0, -'index.html'.length);

  return `${canonicalHost}/${routePath}`;
}

export function sitemapLocationToDistFile(location, canonicalHost) {
  if (!location.startsWith(canonicalHost)) {
    fail(`sitemap.xml location must start with ${canonicalHost}: ${location}`);
  }

  const routePath = location.slice(canonicalHost.length).replace(/\/$/, '');
  const relativeRoutePath = routePath === ''
    ? 'index.html'
    : `${routePath.slice(1)}/index.html`;

  return path.join(DIST_DIR, relativeRoutePath);
}

export function readSitemapLocations(invariants) {
  const sitemapPath = path.join(DIST_DIR, 'sitemap.xml');
  const sitemap = readText(sitemapPath);
  const canonicalHost = invariants.sitemap.requireCanonicalHost;

  const locations = [...sitemap.matchAll(/<loc>([^<]+)<\/loc>/g)]
    .map((match) => match[1].trim());

  if (locations.length === 0) {
    fail('dist/sitemap.xml must contain at least one <loc>');
  }

  for (const location of locations) {
    if (!location.startsWith(canonicalHost)) {
      fail(`sitemap.xml location must start with ${canonicalHost}: ${location}`);
    }
  }

  return new Set(locations);
}

export function isExcluded(relativePath, excludedFiles) {
  return new Set(excludedFiles ?? []).has(relativePath);
}


export function hasAttribute(tag, attributeName) {
  return new RegExp(`\\s${attributeName}(\\s|=|>)`, 'i').test(tag);
}

export function countMatches(html, regex) {
  return html.match(regex)?.length ?? 0;
}

export function stripHashAndQuery(value) {
  return value.split('#')[0].split('?')[0];
}

export function isExternalHref(href) {
  return /^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith('//');
}

export function isAssetHref(href) {
  return /\.[a-z0-9]+$/i.test(stripHashAndQuery(href));
}

export function internalHrefToDistPath(href) {
  const cleanHref = stripHashAndQuery(href);

  if (!cleanHref || cleanHref === '/') {
    return path.join(DIST_DIR, 'index.html');
  }

  if (!cleanHref.startsWith('/')) {
    return null;
  }

  if (isAssetHref(cleanHref)) {
    return path.join(DIST_DIR, cleanHref.slice(1));
  }

  const routePath = cleanHref.endsWith('/') ? cleanHref : `${cleanHref}/`;

  return path.join(DIST_DIR, routePath.slice(1), 'index.html');
}

export function hasAccessibleText(tag, html, tagName) {
  const ariaLabel = extractAttribute(tag, 'aria-label');
  const title = extractAttribute(tag, 'title');

  if (ariaLabel || title) {
    return true;
  }

  const tagStart = html.indexOf(tag);

  if (tagStart === -1) {
    return true;
  }

  const closeTag = `</${tagName}>`;
  const tagEnd = html.indexOf(closeTag, tagStart);

  if (tagEnd === -1) {
    return true;
  }

  const inner = html
    .slice(tagStart + tag.length, tagEnd)
    .replace(/<[^>]+>/g, '')
    .trim();

  return inner.length > 0;
}
