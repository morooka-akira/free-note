import { promisify } from 'util';
import { exec } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import { glob } from 'glob';

const execAsync = promisify(exec);
const copyFileAsync = promisify(fs.copyFile);
const mkdirAsync = promisify(fs.mkdir);
const rmAsync = promisify(fs.rm);

async function buildOrCopyFiles(srcDir: string, distDir: string) {
  try {
    await rmAsync(distDir, { recursive: true, force: true });
    await mkdirAsync(distDir, { recursive: true });
    const files = await glob(`${srcDir}/**/*`, { nodir: true });
    console.log('Files:', files);
    for (const file of files) {
      const ext = path.extname(file);
      const fullPathSrc = path.resolve(file);
      const relativePath = path.relative(srcDir, file);
      const distPath = path.resolve(distDir, relativePath);
      await mkdirAsync(path.dirname(distPath), { recursive: true });
      if (ext === '.ts') {
        await execAsync(`bun build ${fullPathSrc} --output ${distPath.replace('.ts', '.js')}`);
      } else {
        await copyFileAsync(fullPathSrc, distPath);
      }
    }
  } catch (error) {
    console.error('Error processing files:', error);
  }
}

const srcDir = './src';
const distDir = './dist';

buildOrCopyFiles(srcDir, distDir);