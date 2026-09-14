import path from 'node:path';

import { projectPaths } from './project-config.mjs';
import {
  readJsonResource,
} from './project-resource-readers.mjs';

const contentDir = projectPaths.contentDir;

function readContentPagesData(contentIndex) {
  const pagesSection = contentIndex.data?.sections?.pages ?? null;
  const pagesIndexPath = pagesSection?.indexPath
    ? path.join(contentDir, pagesSection.indexPath)
    : null;

  const pagesIndex = pagesIndexPath
    ? readJsonResource(contentDir, pagesIndexPath)
    : null;

  const pagesDir = pagesIndexPath ? path.dirname(pagesIndexPath) : null;
  const pageIds = Array.isArray(pagesIndex?.data?.pageIds) ? pagesIndex.data.pageIds : [];

  const pageIndexes = [];
  const pageLocales = [];

  for (const pageId of pageIds) {
    const pageDir = path.join(pagesDir, pageId);
    const pageIndexPath = path.join(pageDir, 'index.json');
    const pageIndex = readJsonResource(contentDir, pageIndexPath);

    pageIndexes.push({
      pageId,
      ...pageIndex,
    });

    const locales = pageIndex.data?.locales
      && typeof pageIndex.data.locales === 'object'
      && !Array.isArray(pageIndex.data.locales)
      ? pageIndex.data.locales
      : {};

    for (const locale of Object.keys(locales)) {
      const localePath = path.join(pageDir, `${locale}.json`);
      const localeContent = readJsonResource(contentDir, localePath);

      pageLocales.push({
        pageId,
        locale,
        localeIndex: locales[locale],
        ...localeContent,
      });
    }
  }

  return {
    section: pagesSection,
    index: pagesIndex,
    pageIndexes,
    pageLocales,
  };
}

function readContentBlogData(contentIndex) {
  const blogSection = contentIndex.data?.sections?.blog ?? null;
  const blogIndexPath = blogSection?.indexPath
    ? path.join(contentDir, blogSection.indexPath)
    : null;

  const blogIndex = blogIndexPath
    ? readJsonResource(contentDir, blogIndexPath)
    : null;

  const blogDir = blogIndexPath ? path.dirname(blogIndexPath) : null;
  const postPaths = Array.isArray(blogIndex?.data?.postPaths) ? blogIndex.data.postPaths : [];

  return {
    section: blogSection,
    index: blogIndex,
    posts: postPaths.map((postPath) => {
      const postFilePath = path.join(blogDir, postPath);

      return {
        postPath,
        ...readJsonResource(contentDir, postFilePath),
      };
    }),
  };
}

export function readContentData(projectFiles) {
  const index = readJsonResource(contentDir, projectPaths.contentIndex);

  return {
    rootDir: contentDir,
    exists: projectFiles.content.exists,
    files: projectFiles.content.files,
    jsonFiles: projectFiles.content.jsonFiles,
    index,
    pages: readContentPagesData(index),
    blog: readContentBlogData(index),
    redirects: readJsonResource(contentDir, projectPaths.redirects),
    goneRoutes: readJsonResource(contentDir, projectPaths.goneRoutes),
    invariants: readJsonResource(contentDir, projectPaths.siteOutputInvariants),
    websiteMetadata: readJsonResource(contentDir, projectPaths.websiteMetadata),
  };
}
