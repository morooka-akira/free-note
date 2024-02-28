import { expandGlob } from "https://deno.land/std/fs/mod.ts";
import { dirname, extname, relative, resolve } from "https://deno.land/std/path/mod.ts";
import { transpile } from "https://deno.land/x/emit/mod.ts";

async function buildOrCopyFiles(srcDir: string, distDir: string) {
  try {
    // ディレクトリが存在する場合は削除
    try {
      await Deno.remove(distDir, { recursive: true });
    } catch (error) {
      if (!(error instanceof Deno.errors.NotFound)) {
        throw error;
      }
    }
    // 出力ディレクトリを作成
    await Deno.mkdir(distDir, { recursive: true });

    const manifestSrcPath = resolve('./manifest.json');
    const manifestDistPath = resolve(distDir, 'manifest.json');
    await Deno.copyFile(manifestSrcPath, manifestDistPath);

    for await (const file of expandGlob(`${srcDir}/**/*`, { globstar: true })) {
      if (file.isFile) {
        const ext = extname(file.path);
        const fullPathSrc = resolve(file.path);
        const relativePath = relative(srcDir, file.path);
        const distPath = resolve(distDir, relativePath);
        await Deno.mkdir(dirname(distPath), { recursive: true });
        if (ext === '.ts') {
          console.log('transpile', file.path)
          const url = new URL(fullPathSrc, import.meta.url);
          const result = await transpile(url);
          console.log(result);
          for (const code of result.values()) {
            await Deno.writeTextFile(distPath.replace('.ts', '.js'), code);
          } 
        } else {
          // それ以外のファイルはコピー
          await Deno.copyFile(fullPathSrc, distPath);
        }
      }
    }
  } catch (error) {
    console.error('Error processing files:', error);
  }
}

const srcDir = './src';
const distDir = './dist';

await buildOrCopyFiles(srcDir, distDir);