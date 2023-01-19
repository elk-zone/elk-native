import { readFile, writeFile } from 'node:fs/promises'
import glob from 'tiny-glob'
import { join } from 'node:path'

const BASE_URL = 'https://github.com/elk-zone/elk-native/releases/download'

async function main() {
  const version = process.argv[2]
  const channel = process.argv[3] // either nightly or stable

  console.log(process.env.CHANGELOG);

  const obj = {
    name: version,
    notes: process.env.CHANGELOG,
    // notes: (await readFile('RELEASE_NOTES.txt', 'utf-8')).replace('# Version Updates\n\nMerging this PR will release new versions of the following packages based on your change files.\n\n\n\n\n# elk-native\n\n', ''),
    pub_date: new Date().toISOString(),
    platforms: {
      'darwin-aarch64': await getPlatform(version, 'aarch64', [
        'app.tar.gz',
        'app.tar.gz.sig'
      ]),
      'darwin-x86_64': await getPlatform(version, 'x86_64', [
        'app.tar.gz',
        'app.tar.gz.sig'
      ]),
      'linux-x86_64': await getPlatform(version, 'x86_64', [
        'AppImage.tar.gz',
        'AppImage.tar.gz.sig'
      ]),
      'windows-x86_64': await getPlatform(version, 'x86_64', ['msi.zip', 'msi.zip.sig'])
    }
  }

  writeFile(`latest-${channel}.json`, JSON.stringify(obj, null, 2))
}
main()

async function getPlatform(version, arch, exts) {
  const [artifact, signature] = await glob(`./**/*${arch}.{${exts.join(',')}}`, { cwd: join(process.cwd(), "./artifacts") })

  return {
    url: `${BASE_URL}/elk-native-v${version}/${artifact}`,
    signature: await readFile(join(process.cwd(), './artifacts', signature), 'utf-8')
  }
}
