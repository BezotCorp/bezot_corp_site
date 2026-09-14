import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

export function toPosix(filePath) {
  return filePath.split(path.sep).join('/');
}

export function relativeFrom(baseDir, filePath) {
  return toPosix(path.relative(baseDir, filePath));
}

export function fileExists(filePath) {
  return existsSync(filePath);
}

export function collectFiles(dirPath) {
  if (!fileExists(dirPath)) {
    return [];
  }

  return readdirSync(dirPath).flatMap((entry) => {
    const entryPath = path.join(dirPath, entry);

    return statSync(entryPath).isDirectory()
      ? collectFiles(entryPath)
      : [entryPath];
  });
}

export function readTextFile(filePath) {
  return readFileSync(filePath, 'utf8');
}

export function readJsonFile(filePath) {
  return JSON.parse(readTextFile(filePath));
}
