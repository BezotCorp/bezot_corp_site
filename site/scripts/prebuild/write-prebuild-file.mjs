import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

export function writePrebuildFile(filePath, content) {
  const normalizedContent = content.endsWith('\n') ? content : `${content}\n`;

  if (existsSync(filePath)) {
    const currentContent = readFileSync(filePath, 'utf8');

    if (currentContent === normalizedContent) {
      return {
        filePath,
        changed: false,
      };
    }
  }

  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, normalizedContent);

  return {
    filePath,
    changed: true,
  };
}
