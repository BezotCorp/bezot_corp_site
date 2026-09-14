import { existsSync } from 'node:fs';

import {
  extractAttribute,
  extractTags,
} from '../../project-html-data.mjs';
import { readProjectStructuredData } from '../../project-structured-data.mjs';
import {
  fail,
  finishErrorCollection,
  internalHrefToDistPath,
  isExcluded,
  isExternalHref,
  readInvariants,
  startErrorCollection,
} from '../production-check-utils.mjs';

function displayHtmlPath(htmlFile) {
  return `dist/${htmlFile.relativePath}`;
}

export function assertHtmlReference(htmlFile, invariants) {
  const html = htmlFile.html;
  const htmlRules = invariants.html;
  const displayPath = displayHtmlPath(htmlFile);

  if (!htmlRules.requireInternalLinksToExist) {
    return;
  }

  for (const tag of extractTags(html, 'a')) {
    const href = extractAttribute(tag, 'href');

    if (!href || !href.startsWith('/') || isExternalHref(href)) {
      continue;
    }

    const targetPath = internalHrefToDistPath(href);

    if (targetPath && !existsSync(targetPath)) {
      fail(`${displayPath} internal link points to missing dist target: ${href} -> ${targetPath}`);
    }
  }
}

function runHtmlReferenceChecks() {
  startErrorCollection();

  const invariants = readInvariants();
  const projectData = readProjectStructuredData();
  const htmlFiles = projectData.dist.htmlFiles;

  if (htmlFiles.length === 0) {
    fail('HTML reference checks require HTML files');
  }

  for (const htmlFile of htmlFiles) {
    if (isExcluded(htmlFile.relativePath, invariants.html.excludeFiles)) {
      continue;
    }

    assertHtmlReference(htmlFile, invariants);
  }

  finishErrorCollection('HTML reference checks');

  console.log(`HTML reference checks passed:
- internal links point to existing dist targets`);
}

runHtmlReferenceChecks();
