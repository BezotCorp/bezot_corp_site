import { readProjectStructuredData } from '../project-structured-data.mjs';
import {
  fail,
  finishErrorCollection,
  htmlFileToLocation,
  isExcluded,
  readInvariants,
  startErrorCollection,
} from './production-check-utils.mjs';

function displayHtmlPath(htmlFile) {
  return `dist/${htmlFile.relativePath}`;
}

function normalizeUrl(url) {
  return url.endsWith('/') ? url : `${url}/`;
}

function findMetaContentByName(pageData, name) {
  return pageData.metas.find((meta) => meta.name.toLowerCase() === name.toLowerCase())?.content ?? '';
}

function findCanonical(pageData) {
  return pageData.links.find((link) => link.rel.toLowerCase() === 'canonical')?.href ?? '';
}

function hasNoindex(pageData) {
  const robots = findMetaContentByName(pageData, 'robots');

  return robots
    .split(',')
    .map((value) => value.trim().toLowerCase())
    .includes('noindex');
}

function assertSeo(htmlFile, invariants) {
  const seo = invariants.seo;
  const canonicalHost = invariants.sitemap.requireCanonicalHost;
  const pageData = htmlFile.pageData;
  const displayPath = displayHtmlPath(htmlFile);

  const title = pageData?.title ?? '';
  const description = pageData ? findMetaContentByName(pageData, 'description') : '';
  const canonical = pageData ? findCanonical(pageData) : '';
  const expectedCanonical = htmlFileToLocation(htmlFile.filePath, canonicalHost);
  const h1Count = pageData?.headings.filter((heading) => heading.level === 1).length ?? 0;

  if (seo.requireTitle && !title) {
    fail(`${displayPath} must include a non-empty <title>`);
  }

  if (seo.requireMetaDescription && !description) {
    fail(`${displayPath} must include a non-empty meta description`);
  }

  if (seo.requireCanonical && !canonical) {
    fail(`${displayPath} must include a non-empty canonical link`);
  }

  if (
    seo.requireCanonicalMatchesRoute &&
    expectedCanonical &&
    normalizeUrl(canonical) !== normalizeUrl(expectedCanonical)
  ) {
    fail(
      `${displayPath} canonical must match its route: ${canonical} !== ${expectedCanonical}`,
    );
  }

  if (seo.requireHtmlLang && !pageData?.htmlLang) {
    fail(`${displayPath} must include html lang`);
  }

  if (seo.requireSingleH1 && h1Count !== 1) {
    fail(`${displayPath} must include exactly one <h1>`);
  }

  if (seo.forbidNoindexOnPublishedPages && pageData && hasNoindex(pageData)) {
    fail(`${displayPath} must not include noindex`);
  }
}

function runProductionSeoChecks() {
  startErrorCollection();

  const invariants = readInvariants();
  const projectData = readProjectStructuredData();
  const htmlFiles = projectData.dist.htmlFiles;

  if (htmlFiles.length === 0) {
    fail('Production SEO checks require HTML files');
  }

  for (const htmlFile of htmlFiles) {
    if (isExcluded(htmlFile.relativePath, invariants.seo.excludeFiles)) {
      continue;
    }

    assertSeo(htmlFile, invariants);
  }

  finishErrorCollection('Production SEO checks');

  console.log(`Production SEO checks passed:
- HTML outputs respect configured SEO requirements
- title, meta description and canonical are checked
- canonical URLs match generated routes`);
}

runProductionSeoChecks();
