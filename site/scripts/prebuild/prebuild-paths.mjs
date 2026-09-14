import path from 'node:path';

import { projectRootDir } from '../project-config.mjs';

export const prebuildRootDir = path.join(projectRootDir, 'prebuild');

export const prebuildPaths = {
  rootDir: prebuildRootDir,
  srcDir: path.join(prebuildRootDir, 'src'),

  siteDir: path.join(prebuildRootDir, 'src/site'),
  pagesDir: path.join(prebuildRootDir, 'src/pages'),
  postsDir: path.join(prebuildRootDir, 'src/posts'),
  redirectsDir: path.join(prebuildRootDir, 'src/redirects'),
  goneDir: path.join(prebuildRootDir, 'src/gone'),
  routesDir: path.join(prebuildRootDir, 'src/routes'),
};
