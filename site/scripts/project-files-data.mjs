import { projectRootDir, projectPaths } from './project-config.mjs';
import {
  collectFiles,
  fileExists,
} from './project-file-utils.mjs';
import {
  fileRecord,
} from './project-resource-readers.mjs';

function readDirectoryFiles(rootDir) {
  const filePaths = collectFiles(rootDir);
  const files = filePaths.map((filePath) => fileRecord(rootDir, filePath));

  return {
    rootDir,
    exists: fileExists(rootDir),
    files,
  };
}

export function readProjectFilesData() {
  const content = readDirectoryFiles(projectPaths.contentDir);
  const dist = readDirectoryFiles(projectPaths.distDir);

  return {
    rootDir: projectRootDir,
    content: {
      ...content,
      jsonFiles: content.files.filter((file) => file.extension === '.json'),
    },
    dist: {
      ...dist,
      htmlFiles: dist.files.filter((file) => file.extension === '.html'),
      xmlFiles: dist.files.filter((file) => file.extension === '.xml'),
    },
  };
}
