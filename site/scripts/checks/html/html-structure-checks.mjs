import {
  extractAttribute,
  extractTags,
} from '../../project-html-data.mjs';
import { readProjectStructuredData } from '../../project-structured-data.mjs';
import {
  fail,
  finishErrorCollection,
  isExcluded,
  readInvariants,
  startErrorCollection,
} from '../production-check-utils.mjs';

function displayHtmlPath(htmlFile) {
  return `dist/${htmlFile.relativePath}`;
}

export function assertHtmlStructure(htmlFile, invariants) {
  const html = htmlFile.html;
  const htmlRules = invariants.html;
  const displayPath = displayHtmlPath(htmlFile);

  if (htmlRules.requireHtmlLang && !/<html\s+[^>]*lang=["'][^"']+["'][^>]*>/i.test(html)) {
    fail(`${displayPath} must include html lang`);
  }

  if (htmlRules.requireSingleH1 && htmlFile.pageData?.headings.filter((heading) => heading.level === 1).length !== 1) {
    fail(`${displayPath} must include exactly one <h1>`);
  }

  for (const tag of extractTags(html, 'a')) {
    const href = extractAttribute(tag, 'href');

    if (!href) {
      fail(`${displayPath} link must include href: ${tag}`);
    }
  }
}

function runHtmlStructureChecks() {
  startErrorCollection();

  const invariants = readInvariants();
  const projectData = readProjectStructuredData();
  const htmlFiles = projectData.dist.htmlFiles;

  if (htmlFiles.length === 0) {
    fail('HTML structure checks require HTML files');
  }

  for (const htmlFile of htmlFiles) {
    if (isExcluded(htmlFile.relativePath, invariants.html.excludeFiles)) {
      continue;
    }

    assertHtmlStructure(htmlFile, invariants);
  }

  finishErrorCollection('HTML structure checks');

  console.log(`HTML structure checks passed:
- HTML outputs include required structural markers
- h1 structure is checked
- links include href attributes`);
}

runHtmlStructureChecks();
