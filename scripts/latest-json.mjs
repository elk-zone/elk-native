import { readFile, writeFile } from 'node:fs/promises'
import glob from 'tiny-glob'
import { join } from 'node:path'

const BASE_URL = 'https://github.com/elk-zone/elk-native/releases/download'

async function main() {
  const version = process.argv[2]
  const channel = process.argv[3] // either nightly or stable

  const obj = {
    name: version,
    notes: process.env.CHANGELOG || '',
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
  const [artifact, signature] = await glob(`./artifacts/**/*${arch}.{${exts.join(',')}}`, { cwd: join(process.cwd(), "./artifacts") })

  return {
    url: `${BASE_URL}/v${version}/${artifact}`,
    signature: await readFile(signature, 'utf-8')
  }
}