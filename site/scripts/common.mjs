import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

export function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf-8'));
}

export function collectJsonFiles(dirPath) {
  if (!existsSync(dirPath)) return [];

  return readdirSync(dirPath)
    .flatMap((entry) => {
      const entryPath = path.join(dirPath, entry);
      return statSync(entryPath).isDirectory()
        ? collectJsonFiles(entryPath)
        : entryPath.endsWith('.json')
          ? [entryPath]
          : [];
    })
    .sort();
}