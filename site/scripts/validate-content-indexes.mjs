import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptsDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptsDir, '..');
const contentDir = path.join(rootDir, 'content');

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function assertFile(filePath) {
  if (!existsSync(filePath)) {
    throw new Error(`Missing file: ${filePath}`);
  }
}

function assertString(value, label) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label} must be a non-empty string`);
  }
}

function assertArray(value, label) {
  if (!Array.isArray(value)) {
    throw new Error(`${label} must be an array`);
  }
}

function assertObject(value, label) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error(`${label} must be an object`);
  }
}

function assertStatus(value, allowed, label) {
  if (!allowed.includes(value)) {
    throw new Error(`${label} has invalid status "${value}". Expected: ${allowed.join(', ')}`);
  }
}

function assertUnique(values, label) {
  const seen = new Set();

  for (const value of values) {
    if (seen.has(value)) {
      throw new Error(`${label} contains duplicate value "${value}"`);
    }

    seen.add(value);
  }
}

function assertSafeRelativePath(relativePath, label) {
  assertString(relativePath, label);

  if (path.isAbsolute(relativePath)) {
    throw new Error(`${label} must be relative, got absolute path "${relativePath}"`);
  }

  if (relativePath.split('/').includes('..')) {
    throw new Error(`${label} must not escape its index directory: "${relativePath}"`);
  }
}

function assertNoLocaleIndexFields(pageId, locale, localeContent) {
  for (const field of ['status', 'updatedAt']) {
    if (Object.hasOwn(localeContent, field)) {
      throw new Error(
        `Page "${pageId}" locale "${locale}" must not define "${field}". Put it in content/pages/${pageId}/index.json.`,
      );
    }
  }
}

const contentIndexPath = path.join(contentDir, 'index.json');
assertFile(contentIndexPath);

const contentIndex = readJson(contentIndexPath);
const pagesSection = contentIndex.sections?.pages;
const blogSection = contentIndex.sections?.blog;

if (!pagesSection) throw new Error('Missing sections.pages in content/index.json');
if (!blogSection) throw new Error('Missing sections.blog in content/index.json');

assertStatus(pagesSection.status, ['enabled', 'disabled'], 'sections.pages');
assertStatus(blogSection.status, ['enabled', 'disabled'], 'sections.blog');

assertSafeRelativePath(pagesSection.indexPath, 'sections.pages.indexPath');
assertSafeRelativePath(blogSection.indexPath, 'sections.blog.indexPath');
assertString(pagesSection.homePageId, 'sections.pages.homePageId');
assertString(blogSection.entryPageId, 'sections.blog.entryPageId');

const pagesIndexPath = path.join(contentDir, pagesSection.indexPath);
assertFile(pagesIndexPath);

const pagesIndex = readJson(pagesIndexPath);
const pagesDir = path.dirname(pagesIndexPath);

assertArray(pagesIndex.pageIds, 'content/pages/index.json pageIds');
assertUnique(pagesIndex.pageIds, 'content/pages/index.json pageIds');

if (!pagesIndex.pageIds.includes(pagesSection.homePageId)) {
  throw new Error(`homePageId "${pagesSection.homePageId}" is not listed in content/pages/index.json`);
}

if (!pagesIndex.pageIds.includes(blogSection.entryPageId)) {
  throw new Error(`blog entryPageId "${blogSection.entryPageId}" is not listed in content/pages/index.json`);
}

for (const pageId of pagesIndex.pageIds) {
  assertString(pageId, 'pageId');

  const pageDir = path.join(pagesDir, pageId);
  const pageIndexPath = path.join(pageDir, 'index.json');

  assertFile(pageIndexPath);

  const pageIndex = readJson(pageIndexPath);
  const locales = pageIndex.locales ?? {};

  for (const [locale, localeIndex] of Object.entries(locales)) {
    assertStatus(localeIndex.status, ['published', 'draft'], `page "${pageId}" locale "${locale}"`);
    assertString(localeIndex.updatedAt, `page "${pageId}" locale "${locale}" updatedAt`);

    const localePath = path.join(pageDir, `${locale}.json`);
    assertFile(localePath);

    const localeContent = readJson(localePath);
    assertNoLocaleIndexFields(pageId, locale, localeContent);
  }
}

const blogIndexPath = path.join(contentDir, blogSection.indexPath);
assertFile(blogIndexPath);

const blogIndex = readJson(blogIndexPath);
const blogDir = path.dirname(blogIndexPath);

assertStatus(blogIndex.status, ['enabled', 'disabled'], 'blog index');
assertString(blogIndex.entryPageId, 'content/blog/index.json entryPageId');

if (blogIndex.entryPageId !== blogSection.entryPageId) {
  throw new Error('Blog entryPageId mismatch between content/index.json and content/blog/index.json');
}

assertArray(blogIndex.postPaths, 'content/blog/index.json postPaths');
assertUnique(blogIndex.postPaths, 'content/blog/index.json postPaths');

for (const postPath of blogIndex.postPaths) {
  assertSafeRelativePath(postPath, `blog post path "${postPath}"`);

  const postFilePath = path.join(blogDir, postPath);
  assertFile(postFilePath);

  const post = readJson(postFilePath);

  assertString(post.id, `blog post "${postPath}" id`);
  assertStatus(post.status, ['published', 'draft', 'archived'], `blog post "${post.id}"`);
  assertString(post.publishedAt, `blog post "${post.id}" publishedAt`);
  assertObject(post.locales, `blog post "${post.id}" locales`);
}

console.log('Content indexes are valid');
