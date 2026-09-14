import { projectPaths } from './project-config.mjs';
import {
  readTextResource,
} from './project-resource-readers.mjs';
import {
  extractHtmlPageData,
  htmlRelativePathToRoute,
} from './project-html-data.mjs';

const distDir = projectPaths.distDir;

export function readDistData(projectFiles) {
  const sitemap = readTextResource(distDir, projectPaths.distSitemap);

  const htmlFiles = projectFiles.dist.htmlFiles.map((file) => {
    const textResource = readTextResource(distDir, file.filePath);
    const html = textResource.text ?? '';

    return {
      filePath: file.filePath,
      relativePath: file.relativePath,
      route: htmlRelativePathToRoute(file.relativePath),
      exists: textResource.exists,
      html,
      error: textResource.error,
      pageData: html ? extractHtmlPageData(html) : null,
    };
  });

  return {
    rootDir: distDir,
    exists: projectFiles.dist.exists,
    files: projectFiles.dist.files,
    htmlFiles,
    xmlFiles: projectFiles.dist.xmlFiles,
    sitemap: {
      ...sitemap,
      locations: sitemap.text
        ? [...sitemap.text.matchAll(/<loc>([^<]+)<\/loc>/g)].map((match) => match[1].trim())
        : [],
    },
  };
}
