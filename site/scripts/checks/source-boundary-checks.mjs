import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { projectPaths } from '../project-config.mjs';

const forbiddenLayers = new Set(['assembled', 'composition', 'dist', 'generated', 'prebuild']);
const sourceExtensions = new Set(['.ts', '.tsx']);
const violations = [];

function collectSourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const entryPath = path.join(directory, entry.name);

    if (entry.isDirectory()) {
      return collectSourceFiles(entryPath);
    }

    return sourceExtensions.has(path.extname(entry.name)) ? [entryPath] : [];
  });
}

function importedSpecifiers(source) {
  const staticImports = [...source.matchAll(/(?:from\s+|import\s*)['"]([^'"]+)['"]/g)];
  const dynamicImports = [...source.matchAll(/import\s*\(\s*['"]([^'"]+)['"]\s*\)/g)];
  return [...staticImports, ...dynamicImports].map((match) => match[1]);
}

for (const filePath of collectSourceFiles(projectPaths.srcDir)) {
  const source = readFileSync(filePath, 'utf8');

  for (const specifier of importedSpecifiers(source)) {
    const segments = specifier
      .split('/')
      .filter((segment) => segment && segment !== '..' && segment !== '.');
    const forbiddenLayer = segments.find((segment) => forbiddenLayers.has(segment));

    if (forbiddenLayer) {
      violations.push(
        `${path.relative(projectPaths.srcDir, filePath)} imports forbidden layer "${forbiddenLayer}" through "${specifier}"`,
      );
    }
  }
}

if (violations.length > 0) {
  throw new Error(`Source boundary checks failed:\n- ${violations.join('\n- ')}`);
}

console.log('Source boundary checks passed: site/src is independent from assembly outputs');
