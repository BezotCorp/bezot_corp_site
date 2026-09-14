import {
  extractAttribute,
  extractTags,
} from '../../project-html-data.mjs';
import { readProjectStructuredData } from '../../project-structured-data.mjs';
import {
  fail,
  finishErrorCollection,
  hasAccessibleText,
  hasAttribute,
  isExcluded,
  readInvariants,
  startErrorCollection,
} from '../production-check-utils.mjs';

function displayHtmlPath(htmlFile) {
  return `dist/${htmlFile.relativePath}`;
}

export function assertHtmlAccessibility(htmlFile, invariants) {
  const html = htmlFile.html;
  const htmlRules = invariants.html;
  const displayPath = displayHtmlPath(htmlFile);

  if (htmlRules.requireImageAlt) {
    for (const imgTag of extractTags(html, 'img')) {
      if (!hasAttribute(imgTag, 'alt')) {
        fail(`${displayPath} image must include alt: ${imgTag}`);
      }
    }
  }

  if (htmlRules.forbidEmptyAriaLabel) {
    const ariaLabelTags = html.match(/<[^>]+\saria-label=["'][^"']*["'][^>]*>/gi) ?? [];

    for (const tag of ariaLabelTags) {
      if (!extractAttribute(tag, 'aria-label')) {
        fail(`${displayPath} must not include empty aria-label: ${tag}`);
      }
    }
  }

  if (htmlRules.forbidFocusableAriaHidden) {
    const ariaHiddenTags = html.match(/<[^>]+\saria-hidden=["']true["'][^>]*>/gi) ?? [];

    for (const tag of ariaHiddenTags) {
      const isFocusable =
        /<a\s/i.test(tag) ||
        /<button\s/i.test(tag) ||
        /<input\s/i.test(tag) ||
        /<select\s/i.test(tag) ||
        /<textarea\s/i.test(tag) ||
        /tabindex=["']?0["']?/i.test(tag);

      if (isFocusable) {
        fail(`${displayPath} must not hide focusable element with aria-hidden=true: ${tag}`);
      }
    }
  }

  for (const tag of extractTags(html, 'a')) {
    if (!hasAccessibleText(tag, html, 'a')) {
      fail(`${displayPath} link must include accessible text: ${tag}`);
    }
  }

  for (const tag of extractTags(html, 'button')) {
    if (!hasAccessibleText(tag, html, 'button')) {
      fail(`${displayPath} button must include accessible text: ${tag}`);
    }
  }
}

function runHtmlAccessibilityChecks() {
  startErrorCollection();

  const invariants = readInvariants();
  const projectData = readProjectStructuredData();
  const htmlFiles = projectData.dist.htmlFiles;

  if (htmlFiles.length === 0) {
    fail('HTML accessibility checks require HTML files');
  }

  for (const htmlFile of htmlFiles) {
    if (isExcluded(htmlFile.relativePath, invariants.html.excludeFiles)) {
      continue;
    }

    assertHtmlAccessibility(htmlFile, invariants);
  }

  finishErrorCollection('HTML accessibility checks');

  console.log(`HTML accessibility checks passed:
- image alt attributes are checked
- aria-label and aria-hidden misuse are checked
- links and buttons expose accessible text`);
}

runHtmlAccessibilityChecks();
