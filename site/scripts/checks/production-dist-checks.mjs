import path from 'node:path';

import { projectPaths } from '../project-config.mjs';
import {
  toPosix,
} from '../project-file-utils.mjs';
import { readProjectStructuredData } from '../project-structured-data.mjs';
import {
  fail,
  finishErrorCollection,
  isExcluded,
  readInvariants,
  readText,
  sitemapLocationToDistFile,
  startErrorCollection,
} from './production-check-utils.mjs';

function normalizedPath(filePath) {
  return toPosix(path.resolve(filePath));
}

function hasFile(filePaths, filePath) {
  return filePaths.has(normalizedPath(filePath));
}

function displayPath(filePath) {
  return toPosix(path.relative(projectPaths.distDir, filePath)).startsWith('..')
    ? toPosix(filePath)
    : `dist/${toPosix(path.relative(projectPaths.distDir, filePath))}`;
}

function routeLocation(route, canonicalHost) {
  if (!route) {
    return null;
  }

  return `${canonicalHost.replace(/\/$/, '')}${route}`;
}

function assertFile(relativePath, distFilePaths) {
  const filePath = path.join(projectPaths.distDir, relativePath);

  if (!hasFile(distFilePaths, filePath)) {
    fail(`Missing required production file: ${displayPath(filePath)}`);
  }
}

function assertForbiddenPathMissing(relativePath, distFilePaths) {
  const filePath = path.join(projectPaths.distDir, relativePath);

  if (hasFile(distFilePaths, filePath)) {
    fail(`Forbidden production path exists: ${displayPath(filePath)}`);
  }
}

function assertNotIncludes(value, forbidden, label) {
  if (value.includes(forbidden)) {
    fail(`${label} must not include: ${forbidden}`);
  }
}

function assertSitemapTargetsExist(locations, invariants, distFilePaths) {
  for (const location of locations) {
    const targetFile = sitemapLocationToDistFile(
      location,
      invariants.sitemap.requireCanonicalHost,
    );

    if (!hasFile(distFilePaths, targetFile)) {
      fail(
        `sitemap.xml points to a missing HTML file: ${location} -> ${toPosix(targetFile)}`,
      );
    }
  }
}

function assertHtmlFilesAreListedInSitemap(htmlFiles, locations, invariants) {
  const excludedFiles = invariants.sitemap.excludeFiles ?? [];

  for (const htmlFile of htmlFiles) {
    if (isExcluded(htmlFile.relativePath, excludedFiles)) {
      continue;
    }

    const location = routeLocation(
      htmlFile.route,
      invariants.sitemap.requireCanonicalHost,
    );

    if (location && !locations.has(location)) {
      fail(`HTML file is missing from sitemap.xml: ${displayPath(htmlFile.filePath)} -> ${location}`);
    }
  }
}

function routeOutputPath(locale, slug) {
  if (!slug) {
    return path.join(projectPaths.distDir, locale, 'index.html');
  }

  return path.join(projectPaths.distDir, locale, slug, 'index.html');
}

function assertPagesMatchContentIndexes(projectData, distFilePaths) {
  const pagesSection = projectData.content.index.data?.sections?.pages;

  if (!pagesSection || pagesSection.status !== 'enabled') {
    return;
  }

  for (const pageIndex of projectData.content.pages.pageIndexes) {
    const localeEntries = Object.entries(pageIndex.data?.locales ?? {});

    if (localeEntries.length === 0) {
      fail(`${toPosix(pageIndex.relativePath)} must declare at least one locale`);
    }

    const publishedLocales = localeEntries
      .filter(([, localeConfig]) => localeConfig.status === 'published')
      .map(([locale]) => locale);

    if (publishedLocales.length === 0) {
      fail(
        `${toPosix(pageIndex.relativePath)} must declare at least one published locale or be removed from pages/index.json`,
      );
    }

    for (const [locale, localeConfig] of localeEntries) {
      const outputPath = routeOutputPath(locale, localeConfig.slug);
      const outputExists = hasFile(distFilePaths, outputPath);

      if (localeConfig.status === 'published' && !outputExists) {
        fail(`Published page is missing from dist: ${pageIndex.pageId}/${locale} -> ${displayPath(outputPath)}`);
      }

      if (localeConfig.status !== 'published' && outputExists) {
        fail(`Non-published page exists in dist: ${pageIndex.pageId}/${locale} -> ${displayPath(outputPath)}`);
      }
    }
  }
}

function postSlugFromPath(postPath) {
  return path.basename(postPath, '.json');
}

function candidatePostOutputPaths(locale, postPath, post, localeConfig) {
  const slug = localeConfig.slug ?? post.slug ?? post.id ?? postSlugFromPath(postPath);

  return [
    routeOutputPath(locale, slug),
  ];
}

function assertBlogPostsMatchContentIndexes(projectData, distFilePaths) {
  const blogSection = projectData.content.index.data?.sections?.blog;

  if (!blogSection || blogSection.status !== 'enabled') {
    return;
  }

  for (const postResource of projectData.content.blog.posts) {
    const post = postResource.data;
    const localeEntries = Object.entries(post?.locales ?? {});

    for (const [locale, localeConfig] of localeEntries) {
      const status = localeConfig.status ?? post.status;
      const isPublished = status === 'published';
      const candidates = candidatePostOutputPaths(locale, postResource.postPath, post, localeConfig);
      const existingCandidates = candidates.filter((candidatePath) => hasFile(distFilePaths, candidatePath));

      if (isPublished && existingCandidates.length === 0) {
        fail(
          `Published blog post is missing from dist: ${postResource.postPath}/${locale} -> expected one of ${candidates.map(displayPath).join(', ')}`,
        );
      }

      if (!isPublished && existingCandidates.length > 0) {
        fail(
          `Non-published blog post exists in dist: ${postResource.postPath}/${locale} -> ${existingCandidates.map(displayPath).join(', ')}`,
        );
      }
    }
  }
}

function runProductionDistChecks() {
  startErrorCollection();

  const invariants = readInvariants();
  const projectData = readProjectStructuredData();
  const distFiles = projectData.files.dist.files;
  const distFilePaths = new Set(distFiles.map((file) => normalizedPath(file.filePath)));

  if (!projectData.files.dist.exists) {
    fail('Missing production directory: dist');
  }

  for (const requiredFile of invariants.dist.requiredFiles) {
    assertFile(requiredFile, distFilePaths);
  }

  for (const forbiddenPath of invariants.dist.forbiddenPaths) {
    assertForbiddenPathMissing(forbiddenPath, distFilePaths);
  }

  const publicTextFiles = distFiles.filter((file) =>
    ['.html', '.xml', '.txt'].includes(file.extension) ||
    path.basename(file.filePath) === '.htaccess',
  );

  for (const file of publicTextFiles) {
    const content = readText(file.filePath);

    for (const forbiddenText of invariants.dist.forbiddenPublicText) {
      assertNotIncludes(content, forbiddenText, displayPath(file.filePath));
    }
  }

  const htmlFiles = projectData.dist.htmlFiles;

  if (htmlFiles.length === 0) {
    fail('Production dist must contain HTML files');
  }

  const sitemapLocations = new Set(projectData.dist.sitemap.locations);

  if (invariants.sitemap.requireLocTargetsToExistInDist) {
    assertSitemapTargetsExist(sitemapLocations, invariants, distFilePaths);
  }

  if (invariants.sitemap.requireHtmlFilesToBeListed) {
    assertHtmlFilesAreListedInSitemap(htmlFiles, sitemapLocations, invariants);
  }

  assertPagesMatchContentIndexes(projectData, distFilePaths);
  assertBlogPostsMatchContentIndexes(projectData, distFilePaths);

  finishErrorCollection('Production dist checks');

  console.log(`Production dist checks passed:
- required production files exist
- forbidden production paths are absent
- public text outputs do not contain forbidden text
- HTML outputs exist
- sitemap locations target existing dist HTML files
- public HTML files are listed in sitemap
- page outputs match content page indexes
- blog post outputs match content blog indexes`);
}

runProductionDistChecks();
