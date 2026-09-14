import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const projectRootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
);

export const projectPaths = {
  contentDir: path.join(projectRootDir, 'content'),
  distDir: path.join(projectRootDir, 'dist'),
  srcDir: path.join(projectRootDir, 'src'),

  contentIndex: path.join(projectRootDir, 'content/index.json'),
  redirects: path.join(projectRootDir, 'content/redirects.json'),
  goneRoutes: path.join(projectRootDir, 'content/gone-routes.json'),
  siteOutputInvariants: path.join(projectRootDir, 'content/site-output-invariants.json'),
  websiteMetadata: path.join(projectRootDir, 'content/website-metadata.json'),

  distSitemap: path.join(projectRootDir, 'dist/sitemap.xml'),
};
