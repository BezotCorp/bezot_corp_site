import path from 'node:path';

import {
  fileExists,
  readJsonFile,
  readTextFile,
  relativeFrom,
} from './project-file-utils.mjs';

export function readTextResource(baseDir, filePath) {
  const resource = {
    filePath,
    relativePath: relativeFrom(baseDir, filePath),
    exists: fileExists(filePath),
    text: null,
    error: null,
  };

  if (!resource.exists) {
    return resource;
  }

  try {
    resource.text = readTextFile(filePath);
  } catch (error) {
    resource.error = error.message;
  }

  return resource;
}

export function readJsonResource(baseDir, filePath) {
  const resource = {
    filePath,
    relativePath: relativeFrom(baseDir, filePath),
    exists: fileExists(filePath),
    data: null,
    error: null,
  };

  if (!resource.exists) {
    return resource;
  }

  try {
    resource.data = readJsonFile(filePath);
  } catch (error) {
    resource.error = error.message;
  }

  return resource;
}

export function fileRecord(baseDir, filePath) {
  return {
    filePath,
    relativePath: relativeFrom(baseDir, filePath),
    extension: path.extname(filePath),
  };
}
